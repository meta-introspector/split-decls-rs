// Generated macro for impl_arb_for_tuples (macro)
macro_rules! Depcrate_arbitraryimpl_arb_for_tuples {
() => {
// Module: crate::arbitrary
// Provides: {"impl_arb_for_tuples"}
// Dependencies: {}
macro_rules ! impl_arb_for_tuples { (@ internal [$ ($ acc : tt ,) *]) => { } ; (@ internal [$ ($ acc : tt ,) *] ($ type_param : ident , $ tuple_index : tt) , $ ($ rest : tt ,) *) => { impl_arb_for_single_tuple ! ($ ($ acc ,) * ($ type_param , $ tuple_index) ,) ; impl_arb_for_tuples ! (@ internal [$ ($ acc ,) * ($ type_param , $ tuple_index) ,] $ ($ rest ,) *) ; } ; ($ (($ type_param : ident , $ tuple_index : tt) ,) *) => { impl_arb_for_tuples ! (@ internal [] $ (($ type_param , $ tuple_index) ,) *) ; } ; }
};
}
