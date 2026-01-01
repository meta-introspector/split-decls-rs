/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_USE_0001
/* FP:tests.rs-0002 */ use super :: * ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_USE_0002
/* FP:tests.rs-0004 */ use crate :: Decoder ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_USE_0003
/* FP:tests.rs-0006 */ use crate :: opaque :: { MAGIC_END_BYTES , MemDecoder } ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0004
/* FP:tests.rs-0008 */ macro_rules ! impl_test_unsigned_leb128 { ($ test_name : ident , $ write_fn_name : ident , $ read_fn_name : ident , $ int_ty : ident) => { # [test] fn $ test_name () { let mut values = Vec :: new () ; let increment = (1 as $ int_ty) << ($ int_ty :: BITS - 8) ; values . extend ((0 .. 256) . map (| i | $ int_ty :: MIN + i * increment)) ; values . push ($ int_ty :: MAX) ; values . extend ((- 500 .. 500) . map (| i | (i as $ int_ty) . wrapping_mul (0x12345789ABCDEFu64 as $ int_ty)) ,) ; let mut stream = Vec :: new () ; let mut buf = Default :: default () ; for & x in & values { let n = $ write_fn_name (& mut buf , x) ; stream . extend (& buf [.. n]) ; } let stream_end = stream . len () ; stream . extend (MAGIC_END_BYTES) ; let mut decoder = MemDecoder :: new (& stream , 0) . unwrap () ; for & expected in & values { let actual = $ read_fn_name (& mut decoder) ; assert_eq ! (expected , actual) ; } assert_eq ! (stream_end , decoder . position ()) ; } } ; }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0005
/* FP:tests.rs-0010 */ impl_test_unsigned_leb128 ! (test_u16_leb128 , write_u16_leb128 , read_u16_leb128 , u16) ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0006
/* FP:tests.rs-0012 */ impl_test_unsigned_leb128 ! (test_u32_leb128 , write_u32_leb128 , read_u32_leb128 , u32) ;
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0007
/* FP:tests.rs-0014 */ impl_test_unsigned_leb128 ! (test_u64_leb128 , write_u64_leb128 , read_u64_leb128 , u64) ;
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0008
/* FP:tests.rs-0016 */ impl_test_unsigned_leb128 ! (test_u128_leb128 , write_u128_leb128 , read_u128_leb128 , u128) ;
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0009
/* FP:tests.rs-0018 */ impl_test_unsigned_leb128 ! (test_usize_leb128 , write_usize_leb128 , read_usize_leb128 , usize) ;
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0010
/* FP:tests.rs-0020 */ macro_rules ! impl_test_signed_leb128 { ($ test_name : ident , $ write_fn_name : ident , $ read_fn_name : ident , $ int_ty : ident) => { # [test] fn $ test_name () { let mut values = Vec :: new () ; let mut value = $ int_ty :: MIN ; let increment = (1 as $ int_ty) << ($ int_ty :: BITS - 8) ; for _ in 0 .. 256 { values . push (value) ; value = value . wrapping_add (increment) ; } values . push ($ int_ty :: MAX) ; values . extend ((- 500 .. 500) . map (| i | (i as $ int_ty) . wrapping_mul (0x12345789ABCDEFi64 as $ int_ty)) ,) ; let mut stream = Vec :: new () ; let mut buf = Default :: default () ; for & x in & values { let n = $ write_fn_name (& mut buf , x) ; stream . extend (& buf [.. n]) ; } let stream_end = stream . len () ; stream . extend (MAGIC_END_BYTES) ; let mut decoder = MemDecoder :: new (& stream , 0) . unwrap () ; for & expected in & values { let actual = $ read_fn_name (& mut decoder) ; assert_eq ! (expected , actual) ; } assert_eq ! (stream_end , decoder . position ()) ; } } ; }
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0011
/* FP:tests.rs-0022 */ impl_test_signed_leb128 ! (test_i16_leb128 , write_i16_leb128 , read_i16_leb128 , i16) ;
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0012
/* FP:tests.rs-0024 */ impl_test_signed_leb128 ! (test_i32_leb128 , write_i32_leb128 , read_i32_leb128 , i32) ;
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0013
/* FP:tests.rs-0026 */ impl_test_signed_leb128 ! (test_i64_leb128 , write_i64_leb128 , read_i64_leb128 , i64) ;
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0014
/* FP:tests.rs-0028 */ impl_test_signed_leb128 ! (test_i128_leb128 , write_i128_leb128 , read_i128_leb128 , i128) ;
/* FP:tests.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_leb128_tests_MACRO_0015
/* FP:tests.rs-0030 */ impl_test_signed_leb128 ! (test_isize_leb128 , write_isize_leb128 , read_isize_leb128 , isize) ;