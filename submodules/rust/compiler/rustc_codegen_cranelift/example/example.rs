mkitem!{extern crate mini_core ;}
mkuse!{use mini_core :: * ;}

macro_rules! abc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abc in module {}", module_path!());
    };
}

mkfn!{
    abc_introspect!();
    pub fn abc (a : u8) -> u8 { a * 2 }
}

macro_rules! bcd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bcd in module {}", module_path!());
    };
}

mkfn!{
    bcd_introspect!();
    pub fn bcd (b : bool , a : u8) -> u8 { if b { a * 2 } else { a * 3 } }
}

macro_rules! call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function call in module {}", module_path!());
    };
}

mkfn!{
    call_introspect!();
    pub fn call () { abc (42) ; }
}

macro_rules! indirect_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function indirect_call in module {}", module_path!());
    };
}

mkfn!{
    indirect_call_introspect!();
    pub fn indirect_call () { let f : fn () = call ; f () ; }
}
mkitem!{mkenum!{pub enum BoolOption { Some (bool) , None , }}}

macro_rules! option_unwrap_or_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function option_unwrap_or in module {}", module_path!());
    };
}

mkfn!{
    option_unwrap_or_introspect!();
    pub fn option_unwrap_or (o : BoolOption , d : bool) -> bool { match o { BoolOption :: Some (b) => b , BoolOption :: None => d , } }
}

macro_rules! ret_42_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ret_42 in module {}", module_path!());
    };
}

mkfn!{
    ret_42_introspect!();
    pub fn ret_42 () -> u8 { 42 }
}

macro_rules! return_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function return_str in module {}", module_path!());
    };
}

mkfn!{
    return_str_introspect!();
    pub fn return_str () -> & 'static str { "hello world" }
}

macro_rules! promoted_val_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function promoted_val in module {}", module_path!());
    };
}

mkfn!{
    promoted_val_introspect!();
    pub fn promoted_val () -> & 'static u8 { & (1 * 2) }
}

macro_rules! cast_ref_to_raw_ptr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cast_ref_to_raw_ptr in module {}", module_path!());
    };
}

mkfn!{
    cast_ref_to_raw_ptr_introspect!();
    pub fn cast_ref_to_raw_ptr (abc : & u8) -> * const u8 { abc as * const u8 }
}

macro_rules! cmp_raw_ptr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cmp_raw_ptr in module {}", module_path!());
    };
}

mkfn!{
    cmp_raw_ptr_introspect!();
    pub fn cmp_raw_ptr (a : * const u8 , b : * const u8) -> bool { a == b }
}

macro_rules! int_cast_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function int_cast in module {}", module_path!());
    };
}

mkfn!{
    int_cast_introspect!();
    pub fn int_cast (a : u16 , b : i16) -> (u8 , u16 , u32 , usize , i8 , i16 , i32 , isize , u8 , u32) { (a as u8 , a as u16 , a as u32 , a as usize , a as i8 , a as i16 , a as i32 , a as isize , b as u8 , b as u32 ,) }
}

macro_rules! char_cast_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function char_cast in module {}", module_path!());
    };
}

mkfn!{
    char_cast_introspect!();
    pub fn char_cast (c : char) -> u8 { c as u8 }
}
mkitem!{mkstruct!{pub struct DebugTuple (()) ;}}

macro_rules! debug_tuple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debug_tuple in module {}", module_path!());
    };
}

mkfn!{
    debug_tuple_introspect!();
    pub fn debug_tuple () -> DebugTuple { DebugTuple (()) }
}

macro_rules! size_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function size_of in module {}", module_path!());
    };
}

mkfn!{
    size_of_introspect!();
    pub fn size_of < T > () -> usize { intrinsics :: size_of :: < T > () }
}

macro_rules! use_size_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function use_size_of in module {}", module_path!());
    };
}

mkfn!{
    use_size_of_introspect!();
    pub fn use_size_of () -> usize { size_of :: < u64 > () }
}

macro_rules! use_copy_intrinsic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function use_copy_intrinsic in module {}", module_path!());
    };
}

mkfn!{
    use_copy_intrinsic_introspect!();
    pub unsafe fn use_copy_intrinsic (src : * const u8 , dst : * mut u8) { intrinsics :: copy :: < u8 > (src , dst , 1) ; }
}

macro_rules! use_copy_intrinsic_ref_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function use_copy_intrinsic_ref in module {}", module_path!());
    };
}

mkfn!{
    use_copy_intrinsic_ref_introspect!();
    pub unsafe fn use_copy_intrinsic_ref (src : * const u8 , dst : * mut u8) { let copy2 = & intrinsics :: copy :: < u8 > ; copy2 (src , dst , 1) ; }
}
mkitem!{pub const ABC : u8 = 6 * 7 ;}

macro_rules! use_const_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function use_const in module {}", module_path!());
    };
}

mkfn!{
    use_const_introspect!();
    pub fn use_const () -> u8 { ABC }
}

macro_rules! call_closure_3arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function call_closure_3arg in module {}", module_path!());
    };
}

mkfn!{
    call_closure_3arg_introspect!();
    pub fn call_closure_3arg () { (| _ , _ , _ | { }) (0u8 , 42u16 , 0u8) }
}

macro_rules! call_closure_2arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function call_closure_2arg in module {}", module_path!());
    };
}

mkfn!{
    call_closure_2arg_introspect!();
    pub fn call_closure_2arg () { (| _ , _ | { }) (0u8 , 42u16) }
}
mkitem!{mkstruct!{pub struct IsNotEmpty ;}}
mkitem!{mkimpl!{impl < 'a , 'b > FnOnce < (& 'a & 'b [u16] ,) > for IsNotEmpty { type Output = (u8 , u8) ; # [inline] extern "rust-call" fn call_once (mut self , arg : (& 'a & 'b [u16] ,)) -> (u8 , u8) { self . call_mut (arg) } }}}
mkitem!{mkimpl!{impl < 'a , 'b > FnMut < (& 'a & 'b [u16] ,) > for IsNotEmpty { # [inline] extern "rust-call" fn call_mut (& mut self , _arg : (& 'a & 'b [u16] ,)) -> (u8 , u8) { (0 , 42) } }}}

macro_rules! call_is_not_empty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function call_is_not_empty in module {}", module_path!());
    };
}

mkfn!{
    call_is_not_empty_introspect!();
    pub fn call_is_not_empty () { IsNotEmpty . call_once ((& (& [0u16] as & [_]) ,)) ; }
}

macro_rules! eq_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eq_char in module {}", module_path!());
    };
}

mkfn!{
    eq_char_introspect!();
    pub fn eq_char (a : char , b : char) -> bool { a == b }
}

macro_rules! transmute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function transmute in module {}", module_path!());
    };
}

mkfn!{
    transmute_introspect!();
    pub unsafe fn transmute (c : char) -> u32 { intrinsics :: transmute (c) }
}

macro_rules! deref_str_ptr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deref_str_ptr in module {}", module_path!());
    };
}

mkfn!{
    deref_str_ptr_introspect!();
    pub unsafe fn deref_str_ptr (s : * const str) -> & 'static str { & * s }
}

macro_rules! use_array_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function use_array in module {}", module_path!());
    };
}

mkfn!{
    use_array_introspect!();
    pub fn use_array (arr : [u8 ; 3]) -> u8 { arr [1] }
}

macro_rules! repeat_array_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function repeat_array in module {}", module_path!());
    };
}

mkfn!{
    repeat_array_introspect!();
    pub fn repeat_array () -> [u8 ; 3] { [0 ; 3] }
}

macro_rules! array_as_slice_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function array_as_slice in module {}", module_path!());
    };
}

mkfn!{
    array_as_slice_introspect!();
    pub fn array_as_slice (arr : & [u8 ; 3]) -> & [u8] { arr }
}

macro_rules! use_ctlz_nonzero_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function use_ctlz_nonzero in module {}", module_path!());
    };
}

mkfn!{
    use_ctlz_nonzero_introspect!();
    pub unsafe fn use_ctlz_nonzero (a : u16) -> u32 { intrinsics :: ctlz_nonzero (a) }
}

macro_rules! ptr_as_usize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ptr_as_usize in module {}", module_path!());
    };
}

mkfn!{
    ptr_as_usize_introspect!();
    pub fn ptr_as_usize (ptr : * const u8) -> usize { ptr as usize }
}

macro_rules! float_cast_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function float_cast in module {}", module_path!());
    };
}

mkfn!{
    float_cast_introspect!();
    pub fn float_cast (a : f32 , b : f64) -> (f64 , f32) { (a as f64 , b as f32) }
}

macro_rules! int_to_float_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function int_to_float in module {}", module_path!());
    };
}

mkfn!{
    int_to_float_introspect!();
    pub fn int_to_float (a : u8 , b : i32) -> (f64 , f32) { (a as f64 , b as f32) }
}

macro_rules! make_array_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_array in module {}", module_path!());
    };
}

mkfn!{
    make_array_introspect!();
    pub fn make_array () -> [u8 ; 3] { [42 , 0 , 5] }
}

macro_rules! some_promoted_tuple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function some_promoted_tuple in module {}", module_path!());
    };
}

mkfn!{
    some_promoted_tuple_introspect!();
    pub fn some_promoted_tuple () -> & 'static (& 'static str , & 'static str) { & ("abc" , "some") }
}

macro_rules! index_slice_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function index_slice in module {}", module_path!());
    };
}

mkfn!{
    index_slice_introspect!();
    pub fn index_slice (s : & [u8]) -> u8 { s [2] }
}
mkitem!{mkstruct!{pub struct StrWrapper { s : str , }}}

macro_rules! str_wrapper_get_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function str_wrapper_get in module {}", module_path!());
    };
}

mkfn!{
    str_wrapper_get_introspect!();
    pub fn str_wrapper_get (w : & StrWrapper) -> & str { & w . s }
}

macro_rules! i16_as_i8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i16_as_i8 in module {}", module_path!());
    };
}

mkfn!{
    i16_as_i8_introspect!();
    pub fn i16_as_i8 (a : i16) -> i8 { a as i8 }
}
mkitem!{mkstruct!{pub struct Unsized (u8 , str) ;}}

macro_rules! get_sized_field_ref_from_unsized_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_sized_field_ref_from_unsized_type in module {}", module_path!());
    };
}

mkfn!{
    get_sized_field_ref_from_unsized_type_introspect!();
    pub fn get_sized_field_ref_from_unsized_type (u : & Unsized) -> & u8 { & u . 0 }
}

macro_rules! get_unsized_field_ref_from_unsized_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_unsized_field_ref_from_unsized_type in module {}", module_path!());
    };
}

mkfn!{
    get_unsized_field_ref_from_unsized_type_introspect!();
    pub fn get_unsized_field_ref_from_unsized_type (u : & Unsized) -> & str { & u . 1 }
}

macro_rules! reuse_byref_argument_storage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reuse_byref_argument_storage in module {}", module_path!());
    };
}

mkfn!{
    reuse_byref_argument_storage_introspect!();
    pub fn reuse_byref_argument_storage (a : (u8 , u16 , u32)) -> u8 { a . 0 }
}