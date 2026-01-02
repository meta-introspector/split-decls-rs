
macro_rules! _Unwind_Resume_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_Resume in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_Resume_introspect!();
    # [no_mangle] unsafe extern "C" fn _Unwind_Resume () { intrinsics :: unreachable () ; }
}
mkitem!{mktrait!{# [lang = "pointee_sized"] pub trait PointeeSized { }}}
mkitem!{mktrait!{# [lang = "meta_sized"] pub trait MetaSized : PointeeSized { }}}
mkitem!{mktrait!{# [lang = "sized"] pub trait Sized : MetaSized { }}}
mkitem!{mktrait!{# [lang = "destruct"] pub trait Destruct { }}}
mkitem!{mktrait!{# [lang = "tuple_trait"] pub trait Tuple { }}}
mkitem!{mktrait!{# [lang = "unsize"] pub trait Unsize < T : PointeeSized > : PointeeSized { }}}
mkitem!{mktrait!{# [lang = "coerce_unsized"] pub trait CoerceUnsized < T > { }}}
mkitem!{mkimpl!{impl < 'a , 'b : 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < & 'a U > for & 'b T { }}}
mkitem!{mkimpl!{impl < 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < & 'a mut U > for & 'a mut T { }}}
mkitem!{mkimpl!{impl < T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < * const U > for * const T { }}}
mkitem!{mkimpl!{impl < T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < * mut U > for * mut T { }}}
mkitem!{mktrait!{# [lang = "dispatch_from_dyn"] pub trait DispatchFromDyn < T > { }}}
mkitem!{mkimpl!{impl < 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < & 'a U > for & 'a T { }}}
mkitem!{mkimpl!{impl < 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < & 'a mut U > for & 'a mut T { }}}
mkitem!{mkimpl!{impl < T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < * const U > for * const T { }}}
mkitem!{mkimpl!{impl < T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < * mut U > for * mut T { }}}
mkitem!{mkimpl!{impl < T : MetaSized + Unsize < U > , U : MetaSized > DispatchFromDyn < Box < U , () > > for Box < T , () > { }}}
mkitem!{mktrait!{# [lang = "legacy_receiver"] pub trait LegacyReceiver { }}}
mkitem!{mkimpl!{impl < T : PointeeSized > LegacyReceiver for & T { }}}
mkitem!{mkimpl!{impl < T : PointeeSized > LegacyReceiver for & mut T { }}}
mkitem!{mkimpl!{impl < T : MetaSized > LegacyReceiver for Box < T > { }}}
mkitem!{mktrait!{# [lang = "receiver"] trait Receiver { }}}
mkitem!{mktrait!{# [lang = "copy"] pub trait Copy { }}}
mkitem!{mktrait!{# [lang = "bikeshed_guaranteed_no_drop"] pub trait BikeshedGuaranteedNoDrop { }}}
mkitem!{mkimpl!{impl Copy for bool { }}}
mkitem!{mkimpl!{impl Copy for u8 { }}}
mkitem!{mkimpl!{impl Copy for u16 { }}}
mkitem!{mkimpl!{impl Copy for u32 { }}}
mkitem!{mkimpl!{impl Copy for u64 { }}}
mkitem!{mkimpl!{impl Copy for usize { }}}
mkitem!{mkimpl!{impl Copy for u128 { }}}
mkitem!{mkimpl!{impl Copy for i8 { }}}
mkitem!{mkimpl!{impl Copy for i16 { }}}
mkitem!{mkimpl!{impl Copy for i32 { }}}
mkitem!{mkimpl!{impl Copy for i64 { }}}
mkitem!{mkimpl!{impl Copy for isize { }}}
mkitem!{mkimpl!{impl Copy for i128 { }}}
mkitem!{mkimpl!{impl Copy for f32 { }}}
mkitem!{mkimpl!{impl Copy for f64 { }}}
mkitem!{mkimpl!{impl Copy for char { }}}
mkitem!{mkimpl!{impl < 'a , T : PointeeSized > Copy for & 'a T { }}}
mkitem!{mkimpl!{impl < T : PointeeSized > Copy for * const T { }}}
mkitem!{mkimpl!{impl < T : PointeeSized > Copy for * mut T { }}}
mkitem!{mktrait!{# [lang = "sync"] pub unsafe trait Sync { }}}
mkitem!{mkimpl!{unsafe impl Sync for bool { }}}
mkitem!{mkimpl!{unsafe impl Sync for u8 { }}}
mkitem!{mkimpl!{unsafe impl Sync for u16 { }}}
mkitem!{mkimpl!{unsafe impl Sync for u32 { }}}
mkitem!{mkimpl!{unsafe impl Sync for u64 { }}}
mkitem!{mkimpl!{unsafe impl Sync for usize { }}}
mkitem!{mkimpl!{unsafe impl Sync for i8 { }}}
mkitem!{mkimpl!{unsafe impl Sync for i16 { }}}
mkitem!{mkimpl!{unsafe impl Sync for i32 { }}}
mkitem!{mkimpl!{unsafe impl Sync for isize { }}}
mkitem!{mkimpl!{unsafe impl Sync for char { }}}
mkitem!{mkimpl!{unsafe impl < 'a , T : PointeeSized > Sync for & 'a T { }}}
mkitem!{mkimpl!{unsafe impl Sync for [u8 ; 16] { }}}
mkitem!{mktrait!{# [lang = "freeze"] unsafe auto trait Freeze { }}}
mkitem!{mkimpl!{unsafe impl < T : PointeeSized > Freeze for PhantomData < T > { }}}
mkitem!{mkimpl!{unsafe impl < T : PointeeSized > Freeze for * const T { }}}
mkitem!{mkimpl!{unsafe impl < T : PointeeSized > Freeze for * mut T { }}}
mkitem!{mkimpl!{unsafe impl < T : PointeeSized > Freeze for & T { }}}
mkitem!{mkimpl!{unsafe impl < T : PointeeSized > Freeze for & mut T { }}}
mkitem!{mktrait!{# [lang = "structural_peq"] pub trait StructuralPartialEq { }}}
mkitem!{mktrait!{# [lang = "not"] pub trait Not { type Output ; fn not (self) -> Self :: Output ; }}}
mkitem!{mkimpl!{impl Not for bool { type Output = bool ; fn not (self) -> bool { ! self } }}}
mkitem!{mktrait!{# [lang = "mul"] pub trait Mul < RHS = Self > { type Output ; # [must_use] fn mul (self , rhs : RHS) -> Self :: Output ; }}}
mkitem!{mkimpl!{impl Mul for u8 { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }}}
mkitem!{mkimpl!{impl Mul for i32 { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }}}
mkitem!{mkimpl!{impl Mul for usize { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }}}
mkitem!{mkimpl!{impl Mul for isize { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }}}
mkitem!{mktrait!{# [lang = "add"] pub trait Add < RHS = Self > { type Output ; fn add (self , rhs : RHS) -> Self :: Output ; }}}
mkitem!{mkimpl!{impl Add for u8 { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }}}
mkitem!{mkimpl!{impl Add for i8 { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }}}
mkitem!{mkimpl!{impl Add for i32 { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }}}
mkitem!{mkimpl!{impl Add for usize { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }}}
mkitem!{mkimpl!{impl Add for isize { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }}}
mkitem!{mktrait!{# [lang = "sub"] pub trait Sub < RHS = Self > { type Output ; fn sub (self , rhs : RHS) -> Self :: Output ; }}}
mkitem!{mkimpl!{impl Sub for usize { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }}}
mkitem!{mkimpl!{impl Sub for isize { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }}}
mkitem!{mkimpl!{impl Sub for u8 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }}}
mkitem!{mkimpl!{impl Sub for i8 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }}}
mkitem!{mkimpl!{impl Sub for i16 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }}}
mkitem!{mkimpl!{impl Sub for i32 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }}}
mkitem!{mktrait!{# [lang = "rem"] pub trait Rem < RHS = Self > { type Output ; fn rem (self , rhs : RHS) -> Self :: Output ; }}}
mkitem!{mkimpl!{impl Rem for usize { type Output = Self ; fn rem (self , rhs : Self) -> Self { self % rhs } }}}
mkitem!{mktrait!{# [lang = "bitor"] pub trait BitOr < RHS = Self > { type Output ; # [must_use] fn bitor (self , rhs : RHS) -> Self :: Output ; }}}
mkitem!{mkimpl!{impl BitOr for bool { type Output = bool ; fn bitor (self , rhs : bool) -> bool { self | rhs } }}}
mkitem!{mkimpl!{impl < 'a > BitOr < bool > for & 'a bool { type Output = bool ; fn bitor (self , rhs : bool) -> bool { * self | rhs } }}}
mkitem!{mktrait!{# [lang = "eq"] pub trait PartialEq < Rhs : ? Sized = Self > { fn eq (& self , other : & Rhs) -> bool ; fn ne (& self , other : & Rhs) -> bool ; }}}
mkitem!{mkimpl!{impl PartialEq for u8 { fn eq (& self , other : & u8) -> bool { (* self) == (* other) } fn ne (& self , other : & u8) -> bool { (* self) != (* other) } }}}
mkitem!{mkimpl!{impl PartialEq for u16 { fn eq (& self , other : & u16) -> bool { (* self) == (* other) } fn ne (& self , other : & u16) -> bool { (* self) != (* other) } }}}
mkitem!{mkimpl!{impl PartialEq for u32 { fn eq (& self , other : & u32) -> bool { (* self) == (* other) } fn ne (& self , other : & u32) -> bool { (* self) != (* other) } }}}
mkitem!{mkimpl!{impl PartialEq for u64 { fn eq (& self , other : & u64) -> bool { (* self) == (* other) } fn ne (& self , other : & u64) -> bool { (* self) != (* other) } }}}
mkitem!{mkimpl!{impl PartialEq for usize { fn eq (& self , other : & usize) -> bool { (* self) == (* other) } fn ne (& self , other : & usize) -> bool { (* self) != (* other) } }}}
mkitem!{mkimpl!{impl PartialEq for i8 { fn eq (& self , other : & i8) -> bool { (* self) == (* other) } fn ne (& self , other : & i8) -> bool { (* self) != (* other) } }}}
mkitem!{mkimpl!{impl PartialEq for i32 { fn eq (& self , other : & i32) -> bool { (* self) == (* other) } fn ne (& self , other : & i32) -> bool { (* self) != (* other) } }}}
mkitem!{mkimpl!{impl PartialEq for isize { fn eq (& self , other : & isize) -> bool { (* self) == (* other) } fn ne (& self , other : & isize) -> bool { (* self) != (* other) } }}}
mkitem!{mkimpl!{impl PartialEq for char { fn eq (& self , other : & char) -> bool { (* self) == (* other) } fn ne (& self , other : & char) -> bool { (* self) != (* other) } }}}
mkitem!{mkimpl!{impl < T : ? Sized > PartialEq for * const T { fn eq (& self , other : & * const T) -> bool { * self == * other } fn ne (& self , other : & * const T) -> bool { * self != * other } }}}
mkitem!{mktrait!{# [lang = "neg"] pub trait Neg { type Output ; fn neg (self) -> Self :: Output ; }}}
mkitem!{mkimpl!{impl Neg for i8 { type Output = i8 ; fn neg (self) -> i8 { - self } }}}
mkitem!{mkimpl!{impl Neg for i16 { type Output = i16 ; fn neg (self) -> i16 { self } }}}
mkitem!{mkimpl!{impl Neg for isize { type Output = isize ; fn neg (self) -> isize { - self } }}}
mkitem!{mkimpl!{impl Neg for f32 { type Output = f32 ; fn neg (self) -> f32 { - self } }}}
mkitem!{mkenum!{pub enum Option < T > { Some (T) , None , }}}
mkuse!{pub use Option :: * ;}
mkitem!{mkstruct!{# [lang = "phantom_data"] pub struct PhantomData < T : PointeeSized > ;}}
mkitem!{mktrait!{# [lang = "fn_once"] # [rustc_paren_sugar] pub trait FnOnce < Args : Tuple > { # [lang = "fn_once_output"] type Output ; extern "rust-call" fn call_once (self , args : Args) -> Self :: Output ; }}}
mkitem!{mktrait!{# [lang = "fn_mut"] # [rustc_paren_sugar] pub trait FnMut < Args : Tuple > : FnOnce < Args > { extern "rust-call" fn call_mut (& mut self , args : Args) -> Self :: Output ; }}}

macro_rules! panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic in module {}", module_path!());
    };
}

mkfn!{
    panic_introspect!();
    # [lang = "panic"] # [track_caller] pub fn panic (_msg : & 'static str) -> ! { unsafe { libc :: puts ("Panicking\n\0" as * const str as * const u8) ; intrinsics :: abort () ; } }
}
mkitem!{macro_rules ! panic_const { ($ ($ lang : ident = $ message : expr ,) +) => { pub mod panic_const { use super ::*; $ (# [track_caller] # [lang = stringify ! ($ lang)] pub fn $ lang () -> ! { panic ($ message) ; }) + } } }}
mkitem!{panic_const ! { panic_const_add_overflow = "attempt to add with overflow" , panic_const_sub_overflow = "attempt to subtract with overflow" , panic_const_mul_overflow = "attempt to multiply with overflow" , panic_const_div_overflow = "attempt to divide with overflow" , panic_const_rem_overflow = "attempt to calculate the remainder with overflow" , panic_const_neg_overflow = "attempt to negate with overflow" , panic_const_shr_overflow = "attempt to shift right with overflow" , panic_const_shl_overflow = "attempt to shift left with overflow" , panic_const_div_by_zero = "attempt to divide by zero" , panic_const_rem_by_zero = "attempt to calculate the remainder with a divisor of zero" , }}

macro_rules! panic_cannot_unwind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_cannot_unwind in module {}", module_path!());
    };
}

mkfn!{
    panic_cannot_unwind_introspect!();
    # [lang = "panic_cannot_unwind"] fn panic_cannot_unwind () -> ! { unsafe { libc :: puts ("Panicking\n\0" as * const str as * const u8) ; intrinsics :: abort () ; } }
}

macro_rules! panic_in_cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_in_cleanup in module {}", module_path!());
    };
}

mkfn!{
    panic_in_cleanup_introspect!();
    # [lang = "panic_in_cleanup"] # [rustc_nounwind] fn panic_in_cleanup () -> ! { unsafe { libc :: printf ("panic in a destructor during cleanup\n\0" as * const str as * const i8) ; intrinsics :: abort () ; } }
}

macro_rules! panic_bounds_check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_bounds_check in module {}", module_path!());
    };
}

mkfn!{
    panic_bounds_check_introspect!();
    # [lang = "panic_bounds_check"] # [track_caller] fn panic_bounds_check (index : usize , len : usize) -> ! { unsafe { libc :: printf ("index out of bounds: the len is %d but the index is %d\n\0" as * const str as * const i8 , len , index ,) ; intrinsics :: abort () ; } }
}

macro_rules! eh_personality_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eh_personality in module {}", module_path!());
    };
}

mkfn!{
    eh_personality_introspect!();
    # [lang = "eh_personality"] fn eh_personality () -> ! { loop { } }
}

macro_rules! drop_in_place_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function drop_in_place in module {}", module_path!());
    };
}

mkfn!{
    drop_in_place_introspect!();
    # [lang = "drop_in_place"] # [allow (unconditional_recursion)] pub unsafe fn drop_in_place < T : ? Sized > (to_drop : * mut T) { drop_in_place (to_drop) ; }
}
mkitem!{mktrait!{# [lang = "unpin"] pub auto trait Unpin { }}}
mkitem!{mktrait!{# [lang = "deref"] pub trait Deref { type Target : ? Sized ; fn deref (& self) -> & Self :: Target ; }}}
mkitem!{mktrait!{pub trait Allocator { }}}
mkitem!{mkimpl!{impl Allocator for () { }}}
mkitem!{mkstruct!{# [lang = "global_alloc_ty"] pub struct Global ;}}
mkitem!{mkimpl!{impl Allocator for Global { }}}
mkitem!{mkstruct!{# [repr (transparent)] # [rustc_layout_scalar_valid_range_start (1)] # [rustc_nonnull_optimization_guaranteed] pub struct NonNull < T : PointeeSized > (pub * const T) ;}}
mkitem!{mkimpl!{impl < T : PointeeSized , U : PointeeSized > CoerceUnsized < NonNull < U > > for NonNull < T > where T : Unsize < U > { }}}
mkitem!{mkimpl!{impl < T : PointeeSized , U : PointeeSized > DispatchFromDyn < NonNull < U > > for NonNull < T > where T : Unsize < U > { }}}
mkitem!{mkstruct!{pub struct Unique < T : PointeeSized > { pub pointer : NonNull < T > , pub _marker : PhantomData < T > , }}}
mkitem!{mkimpl!{impl < T : PointeeSized , U : PointeeSized > CoerceUnsized < Unique < U > > for Unique < T > where T : Unsize < U > { }}}
mkitem!{mkimpl!{impl < T : PointeeSized , U : PointeeSized > DispatchFromDyn < Unique < U > > for Unique < T > where T : Unsize < U > { }}}
mkitem!{mkstruct!{# [lang = "owned_box"] pub struct Box < T : ? Sized , A : Allocator = Global > (Unique < T > , A) ;}}
mkitem!{mkimpl!{impl < T : ? Sized + Unsize < U > , U : ? Sized , A : Allocator > CoerceUnsized < Box < U , A > > for Box < T , A > { }}}
mkitem!{mkimpl!{impl < T > Box < T > { pub fn new (val : T) -> Box < T > { unsafe { let size = intrinsics :: size_of :: < T > () ; let ptr = libc :: malloc (size) ; intrinsics :: copy (& val as * const T as * const u8 , ptr , size) ; Box (Unique { pointer : NonNull (ptr as * const T) , _marker : PhantomData } , Global) } } }}}
mkitem!{mkimpl!{impl < T : ? Sized , A : Allocator > Drop for Box < T , A > { fn drop (& mut self) { unsafe { libc :: free (self . 0 . pointer . 0 as * mut u8) ; } } }}}
mkitem!{mkimpl!{impl < T : ? Sized , A : Allocator > Deref for Box < T , A > { type Target = T ; fn deref (& self) -> & Self :: Target { & * * self } }}}

macro_rules! allocate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function allocate in module {}", module_path!());
    };
}

mkfn!{
    allocate_introspect!();
    # [lang = "exchange_malloc"] unsafe fn allocate (size : usize , _align : usize) -> * mut u8 { libc :: malloc (size) }
}
mkitem!{mktrait!{# [lang = "drop"] pub trait Drop { fn drop (& mut self) ; }}}
mkitem!{mkstruct!{# [lang = "manually_drop"] # [repr (transparent)] pub struct ManuallyDrop < T : ? Sized > { pub value : T , }}}
mkitem!{# [lang = "maybe_uninit"] # [repr (transparent)] pub union MaybeUninit < T > { pub uninit : () , pub value : ManuallyDrop < T > , }}
mkmod!{intrinsics, { 
                getname!(intrinsics);
                getsrc!(intrinsics);
                getpath!(intrinsics);
                get_deps!(intrinsics);
                get_crates!(intrinsics);
                mkinclude!(intrinsics);
                mkitem!{# [rustc_intrinsic] pub const fn black_box < T > (_dummy : T) -> T ;}
mkitem!{# [rustc_intrinsic] pub fn abort () -> !;}
mkitem!{# [rustc_intrinsic] pub fn size_of < T > () -> usize ;}
mkitem!{# [rustc_intrinsic] pub unsafe fn size_of_val < T : ?:: Sized > (val : * const T) -> usize ;}
mkitem!{# [rustc_intrinsic] pub fn align_of < T > () -> usize ;}
mkitem!{# [rustc_intrinsic] pub unsafe fn align_of_val < T : ?:: Sized > (val : * const T) -> usize ;}
mkitem!{# [rustc_intrinsic] pub unsafe fn copy < T > (src : * const T , dst : * mut T , count : usize) ;}
mkitem!{# [rustc_intrinsic] pub unsafe fn transmute < T , U > (e : T) -> U ;}
mkitem!{# [rustc_intrinsic] pub unsafe fn ctlz_nonzero < T > (x : T) -> u32 ;}
mkitem!{# [rustc_intrinsic] pub fn needs_drop < T : ?:: Sized > () -> bool ;}
mkitem!{# [rustc_intrinsic] pub fn bitreverse < T > (x : T) -> T ;}
mkitem!{# [rustc_intrinsic] pub fn bswap < T > (x : T) -> T ;}
mkitem!{# [rustc_intrinsic] pub unsafe fn write_bytes < T > (dst : * mut T , val : u8 , count : usize) ;}
mkitem!{# [rustc_intrinsic] pub unsafe fn unreachable () -> !;} 
            }}
mkmod!{libc, { 
                getname!(libc);
                getsrc!(libc);
                getpath!(libc);
                get_deps!(libc);
                get_crates!(libc);
                mkinclude!(libc);
                mkitem!{# [link (name = "c")] extern "C" { pub fn puts (s : * const u8) -> i32 ; pub fn printf (format : * const i8 , ...) -> i32 ; pub fn malloc (size : usize) -> * mut u8 ; pub fn free (ptr : * mut u8) ; pub fn memcpy (dst : * mut u8 , src : * const u8 , size : usize) ; pub fn memmove (dst : * mut u8 , src : * const u8 , size : usize) ; pub fn strncpy (dst : * mut u8 , src : * const u8 , size : usize) ; pub fn fflush (stream : * mut i32) -> i32 ; pub fn exit (status : i32) ; pub static stdout : * mut i32 ; }} 
            }}
mkitem!{mktrait!{# [lang = "index"] pub trait Index < Idx : ? Sized > { type Output : ? Sized ; fn index (& self , index : Idx) -> & Self :: Output ; }}}
mkitem!{mkimpl!{impl < T > Index < usize > for [T ; 3] { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { & self [index] } }}}
mkitem!{mkimpl!{impl < T > Index < usize > for [T] { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { & self [index] } }}}
mkitem!{extern "C" { type VaListImpl ; }}
mkitem!{mkstruct!{# [lang = "va_list"] # [repr (transparent)] pub struct VaList < 'a > (& 'a mut VaListImpl) ;}}
mkitem!{# [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro stringify ($ ($ t : tt) *) { }}
mkitem!{# [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro file () { }}
mkitem!{# [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro line () { }}
mkitem!{# [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro cfg () { }}
mkitem!{pub static A_STATIC : u8 = 42 ;}
mkitem!{mkstruct!{# [lang = "panic_location"] struct PanicLocation { file : & 'static str , line : u32 , column : u32 , }}}

macro_rules! get_tls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_tls in module {}", module_path!());
    };
}

mkfn!{
    get_tls_introspect!();
    # [no_mangle] pub fn get_tls () -> u8 { # [thread_local] static A : u8 = 42 ; A }
}