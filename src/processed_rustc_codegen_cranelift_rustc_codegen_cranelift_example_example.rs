/* FP:example.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_USE_0001
/* FP:example.rs-0002 */ # [feature (no_core , unboxed_closures)] # [no_core] # [allow (dead_code , unnecessary_transmutes)] use mini_core :: * ;
/* FP:example.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0002
/* FP:example.rs-0004 */ pub fn abc (a : u8) -> u8 { a * 2 }
/* FP:example.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0003
/* FP:example.rs-0006 */ pub fn bcd (b : bool , a : u8) -> u8 { if b { a * 2 } else { a * 3 } }
/* FP:example.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0004
/* FP:example.rs-0008 */ pub fn call () { abc (42) ; }
/* FP:example.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0005
/* FP:example.rs-0010 */ pub fn indirect_call () { let f : fn () = call ; f () ; }
/* FP:example.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_ENUM_0006
/* FP:example.rs-0012 */ pub enum BoolOption { Some (bool) , None , }
/* FP:example.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0007
/* FP:example.rs-0014 */ pub fn option_unwrap_or (o : BoolOption , d : bool) -> bool { match o { BoolOption :: Some (b) => b , BoolOption :: None => d , } }
/* FP:example.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0008
/* FP:example.rs-0016 */ pub fn ret_42 () -> u8 { 42 }
/* FP:example.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0009
/* FP:example.rs-0018 */ pub fn return_str () -> & 'static str { "hello world" }
/* FP:example.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0010
/* FP:example.rs-0020 */ pub fn promoted_val () -> & 'static u8 { & (1 * 2) }
/* FP:example.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0011
/* FP:example.rs-0022 */ pub fn cast_ref_to_raw_ptr (abc : & u8) -> * const u8 { abc as * const u8 }
/* FP:example.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0012
/* FP:example.rs-0024 */ pub fn cmp_raw_ptr (a : * const u8 , b : * const u8) -> bool { a == b }
/* FP:example.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0013
/* FP:example.rs-0026 */ pub fn int_cast (a : u16 , b : i16) -> (u8 , u16 , u32 , usize , i8 , i16 , i32 , isize , u8 , u32) { (a as u8 , a as u16 , a as u32 , a as usize , a as i8 , a as i16 , a as i32 , a as isize , b as u8 , b as u32 ,) }
/* FP:example.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0014
/* FP:example.rs-0028 */ pub fn char_cast (c : char) -> u8 { c as u8 }
/* FP:example.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_STRUCT_0015
/* FP:example.rs-0030 */ pub struct DebugTuple (()) ;
/* FP:example.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0016
/* FP:example.rs-0032 */ pub fn debug_tuple () -> DebugTuple { DebugTuple (()) }
/* FP:example.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0017
/* FP:example.rs-0034 */ pub fn size_of < T > () -> usize { intrinsics :: size_of :: < T > () }
/* FP:example.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0018
/* FP:example.rs-0036 */ pub fn use_size_of () -> usize { size_of :: < u64 > () }
/* FP:example.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0019
/* FP:example.rs-0038 */ pub unsafe fn use_copy_intrinsic (src : * const u8 , dst : * mut u8) { intrinsics :: copy :: < u8 > (src , dst , 1) ; }
/* FP:example.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0020
/* FP:example.rs-0040 */ pub unsafe fn use_copy_intrinsic_ref (src : * const u8 , dst : * mut u8) { let copy2 = & intrinsics :: copy :: < u8 > ; copy2 (src , dst , 1) ; }
/* FP:example.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_CONST_0021
/* FP:example.rs-0042 */ pub const ABC : u8 = 6 * 7 ;
/* FP:example.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0022
/* FP:example.rs-0044 */ pub fn use_const () -> u8 { ABC }
/* FP:example.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0023
/* FP:example.rs-0046 */ pub fn call_closure_3arg () { (| _ , _ , _ | { }) (0u8 , 42u16 , 0u8) }
/* FP:example.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0024
/* FP:example.rs-0048 */ pub fn call_closure_2arg () { (| _ , _ | { }) (0u8 , 42u16) }
/* FP:example.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_STRUCT_0025
/* FP:example.rs-0050 */ pub struct IsNotEmpty ;
/* FP:example.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_IMPL_0026
/* FP:example.rs-0052 */ impl < 'a , 'b > FnOnce < (& 'a & 'b [u16] ,) > for IsNotEmpty { type Output = (u8 , u8) ; # [inline] extern "rust-call" fn call_once (mut self , arg : (& 'a & 'b [u16] ,)) -> (u8 , u8) { self . call_mut (arg) } }
/* FP:example.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_IMPL_0027
/* FP:example.rs-0054 */ impl < 'a , 'b > FnMut < (& 'a & 'b [u16] ,) > for IsNotEmpty { # [inline] extern "rust-call" fn call_mut (& mut self , _arg : (& 'a & 'b [u16] ,)) -> (u8 , u8) { (0 , 42) } }
/* FP:example.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0028
/* FP:example.rs-0056 */ pub fn call_is_not_empty () { IsNotEmpty . call_once ((& (& [0u16] as & [_]) ,)) ; }
/* FP:example.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0029
/* FP:example.rs-0058 */ pub fn eq_char (a : char , b : char) -> bool { a == b }
/* FP:example.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0030
/* FP:example.rs-0060 */ pub unsafe fn transmute (c : char) -> u32 { intrinsics :: transmute (c) }
/* FP:example.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0031
/* FP:example.rs-0062 */ pub unsafe fn deref_str_ptr (s : * const str) -> & 'static str { & * s }
/* FP:example.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0032
/* FP:example.rs-0064 */ pub fn use_array (arr : [u8 ; 3]) -> u8 { arr [1] }
/* FP:example.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0033
/* FP:example.rs-0066 */ pub fn repeat_array () -> [u8 ; 3] { [0 ; 3] }
/* FP:example.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0034
/* FP:example.rs-0068 */ pub fn array_as_slice (arr : & [u8 ; 3]) -> & [u8] { arr }
/* FP:example.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0035
/* FP:example.rs-0070 */ pub unsafe fn use_ctlz_nonzero (a : u16) -> u32 { intrinsics :: ctlz_nonzero (a) }
/* FP:example.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0036
/* FP:example.rs-0072 */ pub fn ptr_as_usize (ptr : * const u8) -> usize { ptr as usize }
/* FP:example.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0037
/* FP:example.rs-0074 */ pub fn float_cast (a : f32 , b : f64) -> (f64 , f32) { (a as f64 , b as f32) }
/* FP:example.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0038
/* FP:example.rs-0076 */ pub fn int_to_float (a : u8 , b : i32) -> (f64 , f32) { (a as f64 , b as f32) }
/* FP:example.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0039
/* FP:example.rs-0078 */ pub fn make_array () -> [u8 ; 3] { [42 , 0 , 5] }
/* FP:example.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0040
/* FP:example.rs-0080 */ pub fn some_promoted_tuple () -> & 'static (& 'static str , & 'static str) { & ("abc" , "some") }
/* FP:example.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0041
/* FP:example.rs-0082 */ pub fn index_slice (s : & [u8]) -> u8 { s [2] }
/* FP:example.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_STRUCT_0042
/* FP:example.rs-0084 */ pub struct StrWrapper { s : str , }
/* FP:example.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0043
/* FP:example.rs-0086 */ pub fn str_wrapper_get (w : & StrWrapper) -> & str { & w . s }
/* FP:example.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0044
/* FP:example.rs-0088 */ pub fn i16_as_i8 (a : i16) -> i8 { a as i8 }
/* FP:example.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_STRUCT_0045
/* FP:example.rs-0090 */ pub struct Unsized (u8 , str) ;
/* FP:example.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0046
/* FP:example.rs-0092 */ pub fn get_sized_field_ref_from_unsized_type (u : & Unsized) -> & u8 { & u . 0 }
/* FP:example.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0047
/* FP:example.rs-0094 */ pub fn get_unsized_field_ref_from_unsized_type (u : & Unsized) -> & str { & u . 1 }
/* FP:example.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_example_FN_0048
/* FP:example.rs-0096 */ pub fn reuse_byref_argument_storage (a : (u8 , u16 , u32)) -> u8 { a . 0 }