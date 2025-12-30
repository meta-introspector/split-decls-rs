// Generated macro for impl_348 (impl)
macro_rules! Depcrate_adjimpl_348 {
() => {
// Module: crate::adj
// Provides: {"impl_348"}
// Dependencies: {}
impl < E , Ix > fmt :: Debug for EdgeReferences < '_ , E , Ix > where E : fmt :: Debug , Ix : IndexType , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut edge_list = f . debug_list () ; let iter : Self = self . clone () ; for e in iter { if core :: mem :: size_of :: < E > () != 0 { edge_list . entry (& (NoPretty ((e . source () . index () , e . target () . index ())) , e . weight () ,)) ; } else { edge_list . entry (& NoPretty ((e . source () . index () , e . target () . index ()))) ; } } edge_list . finish () } }
};
}
