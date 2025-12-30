// Generated macro for create_c_test_function (function)
macro_rules! Depcrate_common_gen_ccreate_c_test_function {
() => {
// Module: crate::common::gen_c
// Provides: {"create_c_test_function"}
// Dependencies: {}
pub fn create_c_test_function < T : IntrinsicTypeDefinition > (w : & mut impl std :: io :: Write , intrinsic : & Intrinsic < T > ,) -> std :: io :: Result < () > { let indentation = Indentation :: default () ; writeln ! (w , "int run_{}() {{" , intrinsic . name) ? ; let arguments = & intrinsic . arguments ; arguments . gen_arglists_c (w , indentation . nested () , PASSES) ? ; generate_c_constraint_blocks (w , intrinsic , indentation . nested () , & mut arguments . iter () . rev () . filter (| & i | i . has_constraint ()) , Default :: default () ,) ? ; writeln ! (w , "    return 0;") ? ; writeln ! (w , "}}") ? ; Ok (()) }
};
}
