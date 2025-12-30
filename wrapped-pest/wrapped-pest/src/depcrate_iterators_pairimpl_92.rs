// Generated macro for impl_92 (impl)
macro_rules! Depcrate_iterators_pairimpl_92 {
() => {
// Module: crate::iterators::pair
// Provides: {"impl_92"}
// Dependencies: {}
impl < R : RuleType > fmt :: Debug for Pair < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let pair = & mut f . debug_struct ("Pair") ; pair . field ("rule" , & self . as_rule ()) ; if let Some (s) = self . as_node_tag () { pair . field ("node_tag" , & s) ; } pair . field ("span" , & self . as_span ()) . field ("inner" , & self . clone () . into_inner () . collect :: < Vec < _ > > ()) . finish () } }
};
}
