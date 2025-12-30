// Generated macro for write_lib_rs (function)
macro_rules! Depcrate_common_gen_rustwrite_lib_rs {
() => {
// Module: crate::common::gen_rust
// Provides: {"write_lib_rs"}
// Dependencies: {}
pub fn write_lib_rs < T : IntrinsicTypeDefinition > (w : & mut impl std :: io :: Write , notice : & str , cfg : & str , definitions : & str , intrinsics : & [Intrinsic < T >] ,) -> std :: io :: Result < () > { write ! (w , "{notice}") ? ; writeln ! (w , "#![feature(simd_ffi)]") ? ; writeln ! (w , "#![feature(f16)]") ? ; writeln ! (w , "#![allow(unused)]") ? ; writeln ! (w , "#![allow(non_upper_case_globals)]") ? ; writeln ! (w , "#![allow(non_camel_case_types)]") ? ; writeln ! (w , "#![allow(non_snake_case)]") ? ; writeln ! (w , "{cfg}") ? ; writeln ! (w , "{definitions}") ? ; let mut seen = std :: collections :: HashSet :: new () ; for intrinsic in intrinsics { for arg in & intrinsic . arguments . args { if ! arg . has_constraint () && arg . ty . is_rust_vals_array_const () { let name = arg . rust_vals_array_name () . to_string () ; if seen . insert (name) { ArgumentList :: gen_arg_rust (arg , w , Indentation :: default () , PASSES) ? ; } } } } for intrinsic in intrinsics { crate :: common :: gen_rust :: create_rust_test_module (w , intrinsic) ? ; } Ok (()) }
};
}
