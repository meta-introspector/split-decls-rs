mkuse!{use core :: sync :: atomic :: { AtomicU32 , Ordering } ;}
mkuse!{use core :: { arch , mem } ;}

macro_rules! __kuser_cmpxchg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __kuser_cmpxchg in module {}", module_path!());
    };
}

mkfn!{
    __kuser_cmpxchg_introspect!();
    unsafe fn __kuser_cmpxchg (oldval : u32 , newval : u32 , ptr : * mut u32) -> bool { let f = unsafe { mem :: transmute :: < _ , extern "C" fn (u32 , u32 , * mut u32) -> u32 > (0xffff0fc0usize as * const ()) } ; f (oldval , newval , ptr) == 0 }
}

macro_rules! __kuser_memory_barrier_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __kuser_memory_barrier in module {}", module_path!());
    };
}

mkfn!{
    __kuser_memory_barrier_introspect!();
    unsafe fn __kuser_memory_barrier () { let f = unsafe { mem :: transmute :: < _ , extern "C" fn () > (0xffff0fa0usize as * const ()) } ; f () ; }
}

macro_rules! align_ptr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function align_ptr in module {}", module_path!());
    };
}

mkfn!{
    align_ptr_introspect!();
    fn align_ptr < T > (ptr : * mut T) -> * mut u32 { let ptr_mask = 3 & (4 - mem :: size_of :: < T > ()) ; (ptr as usize & ! ptr_mask) as * mut u32 }
}

macro_rules! get_shift_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_shift_mask in module {}", module_path!());
    };
}

mkfn!{
    get_shift_mask_introspect!();
    fn get_shift_mask < T > (ptr : * mut T) -> (u32 , u32) { let mask = match mem :: size_of :: < T > () { 1 => 0xff , 2 => 0xffff , 4 => 0xffffffff , _ => unreachable ! () , } ; let endian_adjust = if cfg ! (target_endian = "little") { 0 } else { 4 - mem :: size_of :: < T > () as u32 } ; let ptr_mask = 3 & (4 - mem :: size_of :: < T > ()) ; let shift = ((ptr as usize & ptr_mask) as u32 ^ endian_adjust) * 8 ; (shift , mask) }
}

macro_rules! extract_aligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_aligned in module {}", module_path!());
    };
}

mkfn!{
    extract_aligned_introspect!();
    fn extract_aligned (aligned : u32 , shift : u32 , mask : u32) -> u32 { (aligned >> shift) & mask }
}

macro_rules! insert_aligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_aligned in module {}", module_path!());
    };
}

mkfn!{
    insert_aligned_introspect!();
    fn insert_aligned (aligned : u32 , val : u32 , shift : u32 , mask : u32) -> u32 { (aligned & ! (mask << shift)) | ((val & mask) << shift) }
}

macro_rules! atomic_load_aligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function atomic_load_aligned in module {}", module_path!());
    };
}

mkfn!{
    atomic_load_aligned_introspect!();
    # [doc = " Performs a relaxed atomic load of 4 bytes at `ptr`. Some of the bytes are allowed to be out of"] # [doc = " bounds as long as `size_of::<T>()` bytes are in bounds."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `ptr` must be 4-aligned."] # [doc = " - `size_of::<T>()` must be at most 4."] # [doc = " - if `size_of::<T>() == 1`, `ptr` or `ptr` offset by 1, 2 or 3 bytes must be valid for a relaxed"] # [doc = "   atomic read of 1 byte."] # [doc = " - if `size_of::<T>() == 2`, `ptr` or `ptr` offset by 2 bytes must be valid for a relaxed atomic"] # [doc = "   read of 2 bytes."] # [doc = " - if `size_of::<T>() == 4`, `ptr` must be valid for a relaxed atomic read of 4 bytes."] unsafe fn atomic_load_aligned < T > (ptr : * mut u32) -> u32 { const { assert ! (size_of ::< T > () <= 4) } ; if size_of :: < T > () == 4 { unsafe { AtomicU32 :: from_ptr (ptr) . load (Ordering :: Relaxed) } } else { unsafe { let res : u32 ; arch :: asm ! ("ldr {res}, [{ptr}]" , ptr = in (reg) ptr , res = lateout (reg) res , options (nostack , preserves_flags , readonly)) ; res } } }
}

macro_rules! atomic_rmw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function atomic_rmw in module {}", module_path!());
    };
}

mkfn!{
    atomic_rmw_introspect!();
    unsafe fn atomic_rmw < T , F : Fn (u32) -> u32 , G : Fn (u32 , u32) -> u32 > (ptr : * mut T , f : F , g : G) -> u32 { let aligned_ptr = align_ptr (ptr) ; let (shift , mask) = get_shift_mask (ptr) ; loop { let curval_aligned = unsafe { atomic_load_aligned :: < T > (aligned_ptr) } ; let curval = extract_aligned (curval_aligned , shift , mask) ; let newval = f (curval) ; let newval_aligned = insert_aligned (curval_aligned , newval , shift , mask) ; if unsafe { __kuser_cmpxchg (curval_aligned , newval_aligned , aligned_ptr) } { return g (curval , newval) ; } } }
}

macro_rules! atomic_cmpxchg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function atomic_cmpxchg in module {}", module_path!());
    };
}

mkfn!{
    atomic_cmpxchg_introspect!();
    unsafe fn atomic_cmpxchg < T > (ptr : * mut T , oldval : u32 , newval : u32) -> u32 { let aligned_ptr = align_ptr (ptr) ; let (shift , mask) = get_shift_mask (ptr) ; loop { let curval_aligned = unsafe { atomic_load_aligned :: < T > (aligned_ptr) } ; let curval = extract_aligned (curval_aligned , shift , mask) ; if curval != oldval { return curval ; } let newval_aligned = insert_aligned (curval_aligned , newval , shift , mask) ; if unsafe { __kuser_cmpxchg (curval_aligned , newval_aligned , aligned_ptr) } { return oldval ; } } }
}
mkitem!{macro_rules ! atomic_rmw { ($ name : ident , $ ty : ty , $ op : expr , $ fetch : expr) => { intrinsics ! { pub unsafe extern "C" fn $ name (ptr : * mut $ ty , val : $ ty) -> $ ty { unsafe { atomic_rmw (ptr , | x | $ op (x as $ ty , val) as u32 , | old , new | $ fetch (old , new)) as $ ty } } } } ; (@ old $ name : ident , $ ty : ty , $ op : expr) => { atomic_rmw ! ($ name , $ ty , $ op , | old , _ | old) ; } ; (@ new $ name : ident , $ ty : ty , $ op : expr) => { atomic_rmw ! ($ name , $ ty , $ op , | _ , new | new) ; } ; }}
mkitem!{macro_rules ! atomic_cmpxchg { ($ name : ident , $ ty : ty) => { intrinsics ! { pub unsafe extern "C" fn $ name (ptr : * mut $ ty , oldval : $ ty , newval : $ ty) -> $ ty { unsafe { atomic_cmpxchg (ptr , oldval as u32 , newval as u32) as $ ty } } } } ; }}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_add_1 , u8 , | a : u8 , b : u8 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_add_2 , u16 , | a : u16 , b : u16 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_add_4 , u32 , | a : u32 , b : u32 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_add_and_fetch_1 , u8 , | a : u8 , b : u8 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_add_and_fetch_2 , u16 , | a : u16 , b : u16 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_add_and_fetch_4 , u32 , | a : u32 , b : u32 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_sub_1 , u8 , | a : u8 , b : u8 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_sub_2 , u16 , | a : u16 , b : u16 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_sub_4 , u32 , | a : u32 , b : u32 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_sub_and_fetch_1 , u8 , | a : u8 , b : u8 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_sub_and_fetch_2 , u16 , | a : u16 , b : u16 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_sub_and_fetch_4 , u32 , | a : u32 , b : u32 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_and_1 , u8 , | a : u8 , b : u8 | a & b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_and_2 , u16 , | a : u16 , b : u16 | a & b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_and_4 , u32 , | a : u32 , b : u32 | a & b) ;}
mkitem!{atomic_rmw ! (@ new __sync_and_and_fetch_1 , u8 , | a : u8 , b : u8 | a & b) ;}
mkitem!{atomic_rmw ! (@ new __sync_and_and_fetch_2 , u16 , | a : u16 , b : u16 | a & b) ;}
mkitem!{atomic_rmw ! (@ new __sync_and_and_fetch_4 , u32 , | a : u32 , b : u32 | a & b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_or_1 , u8 , | a : u8 , b : u8 | a | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_or_2 , u16 , | a : u16 , b : u16 | a | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_or_4 , u32 , | a : u32 , b : u32 | a | b) ;}
mkitem!{atomic_rmw ! (@ new __sync_or_and_fetch_1 , u8 , | a : u8 , b : u8 | a | b) ;}
mkitem!{atomic_rmw ! (@ new __sync_or_and_fetch_2 , u16 , | a : u16 , b : u16 | a | b) ;}
mkitem!{atomic_rmw ! (@ new __sync_or_and_fetch_4 , u32 , | a : u32 , b : u32 | a | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_xor_1 , u8 , | a : u8 , b : u8 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_xor_2 , u16 , | a : u16 , b : u16 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_xor_4 , u32 , | a : u32 , b : u32 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ new __sync_xor_and_fetch_1 , u8 , | a : u8 , b : u8 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ new __sync_xor_and_fetch_2 , u16 , | a : u16 , b : u16 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ new __sync_xor_and_fetch_4 , u32 , | a : u32 , b : u32 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_nand_1 , u8 , | a : u8 , b : u8 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_nand_2 , u16 , | a : u16 , b : u16 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_nand_4 , u32 , | a : u32 , b : u32 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_nand_and_fetch_1 , u8 , | a : u8 , b : u8 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_nand_and_fetch_2 , u16 , | a : u16 , b : u16 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_nand_and_fetch_4 , u32 , | a : u32 , b : u32 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_max_1 , i8 , | a : i8 , b : i8 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_max_2 , i16 , | a : i16 , b : i16 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_max_4 , i32 , | a : i32 , b : i32 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umax_1 , u8 , | a : u8 , b : u8 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umax_2 , u16 , | a : u16 , b : u16 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umax_4 , u32 , | a : u32 , b : u32 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_min_1 , i8 , | a : i8 , b : i8 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_min_2 , i16 , | a : i16 , b : i16 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_min_4 , i32 , | a : i32 , b : i32 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umin_1 , u8 , | a : u8 , b : u8 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umin_2 , u16 , | a : u16 , b : u16 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umin_4 , u32 , | a : u32 , b : u32 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_lock_test_and_set_1 , u8 , | _ : u8 , b : u8 | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_lock_test_and_set_2 , u16 , | _ : u16 , b : u16 | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_lock_test_and_set_4 , u32 , | _ : u32 , b : u32 | b) ;}
mkitem!{atomic_cmpxchg ! (__sync_val_compare_and_swap_1 , u8) ;}
mkitem!{atomic_cmpxchg ! (__sync_val_compare_and_swap_2 , u16) ;}
mkitem!{atomic_cmpxchg ! (__sync_val_compare_and_swap_4 , u32) ;}
mkitem!{intrinsics ! { pub unsafe extern "C" fn __sync_synchronize () { unsafe { __kuser_memory_barrier () } ; } }}