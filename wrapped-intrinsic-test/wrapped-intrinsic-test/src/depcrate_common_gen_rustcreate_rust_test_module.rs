// Generated macro for create_rust_test_module (function)
macro_rules! Depcrate_common_gen_rustcreate_rust_test_module {
() => {
// Module: crate::common::gen_rust
// Provides: {"create_rust_test_module"}
// Dependencies: {}
pub fn create_rust_test_module < T : IntrinsicTypeDefinition > (w : & mut impl std :: io :: Write , intrinsic : & Intrinsic < T > ,) -> std :: io :: Result < () > { trace ! ("generating `{}`" , intrinsic . name) ; let indentation = Indentation :: default () ; writeln ! (w , "pub fn run_{}() {{" , intrinsic . name) ? ; let arguments = & intrinsic . arguments ; arguments . gen_arglists_rust (w , indentation . nested () , PASSES) ? ; let specializations = generate_rust_specializations (& mut arguments . iter () . filter_map (| i | i . constraint . as_ref () . map (| v | v . iter ())) ,) ; generate_rust_test_loop (w , intrinsic , indentation , & specializations , PASSES) ? ; writeln ! (w , "}}") ? ; Ok (()) }
};
}
