/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_USE_0001
/* FP:tests.rs-0002 */ use std :: fmt :: Debug ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_USE_0002
/* FP:tests.rs-0004 */ use std :: fs ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_USE_0003
/* FP:tests.rs-0006 */ use rustc_macros :: { Decodable_NoContext , Encodable_NoContext } ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_USE_0004
/* FP:tests.rs-0008 */ use crate :: opaque :: { FileEncoder , MemDecoder } ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_USE_0005
/* FP:tests.rs-0010 */ use crate :: { Decodable , Encodable } ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_STRUCT_0006
/* FP:tests.rs-0012 */ # [derive (PartialEq , Clone , Debug , Encodable_NoContext , Decodable_NoContext)] struct Struct { a : () , b : u8 , c : u16 , d : u32 , e : u64 , f : usize , g : i8 , h : i16 , i : i32 , j : i64 , k : isize , l : char , m : String , p : bool , q : Option < u32 > , }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0007
/* FP:tests.rs-0014 */ fn check_round_trip < T : Encodable < FileEncoder > + for < 'a > Decodable < MemDecoder < 'a > > + PartialEq + Debug , > (values : Vec < T > ,) { let tmpfile = tempfile :: NamedTempFile :: new () . unwrap () ; let tmpfile = tmpfile . path () ; let mut encoder = FileEncoder :: new (& tmpfile) . unwrap () ; for value in & values { Encodable :: encode (value , & mut encoder) ; } encoder . finish () . unwrap () ; let data = fs :: read (& tmpfile) . unwrap () ; let mut decoder = MemDecoder :: new (& data [..] , 0) . unwrap () ; for value in values { let decoded = Decodable :: decode (& mut decoder) ; assert_eq ! (value , decoded) ; } }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0008
/* FP:tests.rs-0016 */ # [test] fn test_unit () { check_round_trip (vec ! [() , () , () , ()]) ; }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0009
/* FP:tests.rs-0018 */ # [test] fn test_u8 () { let mut vec = vec ! [] ; for i in u8 :: MIN .. u8 :: MAX { vec . push (i) ; } check_round_trip (vec) ; }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0010
/* FP:tests.rs-0020 */ # [test] fn test_u16 () { for i in [u16 :: MIN , 111 , 3333 , 55555 , u16 :: MAX] { check_round_trip (vec ! [1 , 2 , 3 , i , i , i]) ; } }
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0011
/* FP:tests.rs-0022 */ # [test] fn test_u32 () { check_round_trip (vec ! [1 , 2 , 3 , u32 :: MIN , 0 , 1 , u32 :: MAX , 2 , 1]) ; }
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0012
/* FP:tests.rs-0024 */ # [test] fn test_u64 () { check_round_trip (vec ! [1 , 2 , 3 , u64 :: MIN , 0 , 1 , u64 :: MAX , 2 , 1]) ; }
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0013
/* FP:tests.rs-0026 */ # [test] fn test_usize () { check_round_trip (vec ! [1 , 2 , 3 , usize :: MIN , 0 , 1 , usize :: MAX , 2 , 1]) ; }
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0014
/* FP:tests.rs-0028 */ # [test] fn test_i8 () { let mut vec = vec ! [] ; for i in i8 :: MIN .. i8 :: MAX { vec . push (i) ; } check_round_trip (vec) ; }
/* FP:tests.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0015
/* FP:tests.rs-0030 */ # [test] fn test_i16 () { for i in [i16 :: MIN , - 100 , 0 , 101 , i16 :: MAX] { check_round_trip (vec ! [- 1 , 2 , - 3 , i , i , i , 2]) ; } }
/* FP:tests.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0016
/* FP:tests.rs-0032 */ # [test] fn test_i32 () { check_round_trip (vec ! [- 1 , 2 , - 3 , i32 :: MIN , 0 , 1 , i32 :: MAX , 2 , 1]) ; }
/* FP:tests.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0017
/* FP:tests.rs-0034 */ # [test] fn test_i64 () { check_round_trip (vec ! [- 1 , 2 , - 3 , i64 :: MIN , 0 , 1 , i64 :: MAX , 2 , 1]) ; }
/* FP:tests.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0018
/* FP:tests.rs-0036 */ # [test] fn test_isize () { check_round_trip (vec ! [- 1 , 2 , - 3 , isize :: MIN , 0 , 1 , isize :: MAX , 2 , 1]) ; }
/* FP:tests.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0019
/* FP:tests.rs-0038 */ # [test] fn test_bool () { check_round_trip (vec ! [false , true , true , false , false]) ; }
/* FP:tests.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0020
/* FP:tests.rs-0040 */ # [test] fn test_char () { let vec = vec ! ['a' , 'b' , 'c' , 'd' , 'A' , 'X' , ' ' , '#' , 'Ö' , 'Ä' , 'µ' , '€'] ; check_round_trip (vec) ; }
/* FP:tests.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0021
/* FP:tests.rs-0042 */ # [test] fn test_string () { let vec = vec ! ["abcbuÖeiovÄnameÜavmpßvmea€µsbpnvapeapmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmea€µsbpnvapeapmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmea€µsbpapmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmeabpnvapeapmaebn" . to_string () , "abcbuÖganeiÄnameÜavmpßvmea€µsbpnvapeapmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmea€µsbpmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmea€µnvapeapmaebn" . to_string () ,] ; check_round_trip (vec) ; }
/* FP:tests.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0022
/* FP:tests.rs-0044 */ # [test] fn test_option () { check_round_trip (vec ! [Some (- 1i8)]) ; check_round_trip (vec ! [Some (- 2i16)]) ; check_round_trip (vec ! [Some (- 3i32)]) ; check_round_trip (vec ! [Some (- 4i64)]) ; check_round_trip (vec ! [Some (- 5isize)]) ; let none_i8 : Option < i8 > = None ; check_round_trip (vec ! [none_i8]) ; let none_i16 : Option < i16 > = None ; check_round_trip (vec ! [none_i16]) ; let none_i32 : Option < i32 > = None ; check_round_trip (vec ! [none_i32]) ; let none_i64 : Option < i64 > = None ; check_round_trip (vec ! [none_i64]) ; let none_isize : Option < isize > = None ; check_round_trip (vec ! [none_isize]) ; }
/* FP:tests.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0023
/* FP:tests.rs-0046 */ # [test] fn test_struct () { check_round_trip (vec ! [Struct { a : () , b : 10 , c : 11 , d : 12 , e : 13 , f : 14 , g : 15 , h : 16 , i : 17 , j : 18 , k : 19 , l : 'x' , m : "abc" . to_string () , p : false , q : None , }]) ; check_round_trip (vec ! [Struct { a : () , b : 101 , c : 111 , d : 121 , e : 131 , f : 141 , g : - 15 , h : - 16 , i : - 17 , j : - 18 , k : - 19 , l : 'y' , m : "def" . to_string () , p : true , q : Some (1234567) , }]) ; }
/* FP:tests.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_ENUM_0024
/* FP:tests.rs-0048 */ # [derive (PartialEq , Clone , Debug , Encodable_NoContext , Decodable_NoContext)] enum Enum { Variant1 , Variant2 (usize , u32) , Variant3 { a : i32 , b : char , c : bool } , }
/* FP:tests.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0025
/* FP:tests.rs-0050 */ # [test] fn test_enum () { check_round_trip (vec ! [Enum :: Variant1 , Enum :: Variant2 (1 , 25) , Enum :: Variant3 { a : 3 , b : 'b' , c : false } , Enum :: Variant3 { a : - 4 , b : 'f' , c : true } ,]) ; }
/* FP:tests.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0026
/* FP:tests.rs-0052 */ # [test] fn test_sequence () { let mut vec = vec ! [] ; for i in - 100i64 .. 100i64 { vec . push (i * 100000) ; } check_round_trip (vec ! [vec]) ; }
/* FP:tests.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0027
/* FP:tests.rs-0054 */ # [test] fn test_hash_map () { use std :: collections :: HashMap ; let mut map = HashMap :: new () ; for i in - 100i64 .. 100i64 { map . insert (i * 100000 , i * 10000) ; } check_round_trip (vec ! [map]) ; }
/* FP:tests.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0028
/* FP:tests.rs-0056 */ # [test] fn test_tuples () { check_round_trip (vec ! [('x' , () , false , 5u32)]) ; check_round_trip (vec ! [(9i8 , 10u16 , 15i64)]) ; check_round_trip (vec ! [(- 12i16 , 11u8 , 12usize)]) ; check_round_trip (vec ! [(1234567isize , 100000000000000u64 , 99999999999999i64)]) ; check_round_trip (vec ! [(String :: new () , "some string" . to_string ())]) ; }
/* FP:tests.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0029
/* FP:tests.rs-0058 */ # [test] fn test_unit_like_struct () { # [derive (Encodable_NoContext , Decodable_NoContext , PartialEq , Debug)] struct UnitLikeStruct ; check_round_trip (vec ! [UnitLikeStruct]) ; }
/* FP:tests.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0030
/* FP:tests.rs-0060 */ # [test] fn test_box () { # [derive (Encodable_NoContext , Decodable_NoContext , PartialEq , Debug)] struct A { foo : Box < [bool] > , } let obj = A { foo : Box :: new ([true , false]) } ; check_round_trip (vec ! [obj]) ; }
/* FP:tests.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_opaque_tests_FN_0031
/* FP:tests.rs-0062 */ # [test] fn test_cell () { use std :: cell :: { Cell , RefCell } ; # [derive (Encodable_NoContext , Decodable_NoContext , PartialEq , Debug)] struct A { baz : isize , } # [derive (Encodable_NoContext , Decodable_NoContext , PartialEq , Debug)] struct B { foo : Cell < bool > , bar : RefCell < A > , } let obj = B { foo : Cell :: new (true) , bar : RefCell :: new (A { baz : 2 }) } ; check_round_trip (vec ! [obj]) ; }