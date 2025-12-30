// Generated macro for impl_parser_for_tuples (macro)
macro_rules! Depcrate_internalimpl_parser_for_tuples {
() => {
// Module: crate::internal
// Provides: {"impl_parser_for_tuples"}
// Dependencies: {}
macro_rules ! impl_parser_for_tuples { ($ parser1 : ident $ output1 : ident , $ ($ parser : ident $ output : ident) ,+) => { impl_parser_for_tuples ! (__impl $ parser1 $ output1 ; $ ($ parser $ output) ,+) ; } ; (__impl $ ($ parser : ident $ output : ident) ,+; $ parser1 : ident $ output1 : ident $ (,$ parser2 : ident $ output2 : ident) *) => { impl_parser_for_tuple ! ($ ($ parser $ output) ,+) ; impl_parser_for_tuples ! (__impl $ ($ parser $ output) ,+, $ parser1 $ output1 ; $ ($ parser2 $ output2) ,*) ; } ; (__impl $ ($ parser : ident $ output : ident) ,+;) => { impl_parser_for_tuple ! ($ ($ parser $ output) ,+) ; } }
};
}
