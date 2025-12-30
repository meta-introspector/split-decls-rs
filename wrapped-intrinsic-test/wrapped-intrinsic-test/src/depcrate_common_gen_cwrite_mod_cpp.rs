// Generated macro for write_mod_cpp (function)
macro_rules! Depcrate_common_gen_cwrite_mod_cpp {
() => {
// Module: crate::common::gen_c
// Provides: {"write_mod_cpp"}
// Dependencies: {}
pub fn write_mod_cpp < T : IntrinsicTypeDefinition > (w : & mut impl std :: io :: Write , notice : & str , platform_headers : & [& str] , forward_declarations : & str , intrinsics : & [Intrinsic < T >] ,) -> std :: io :: Result < () > { write ! (w , "{notice}") ? ; for header in COMMON_HEADERS . iter () . chain (platform_headers . iter ()) { writeln ! (w , "#include <{header}>") ? ; } writeln ! (w , "{}" , forward_declarations) ? ; for intrinsic in intrinsics { create_c_test_function (w , intrinsic) ? ; } Ok (()) }
};
}
