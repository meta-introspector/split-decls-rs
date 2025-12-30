// Generated macro for impl_241 (impl)
macro_rules! Depcrate_builder_rangeimpl_241 {
() => {
// Module: crate::builder::range
// Provides: {"impl_241"}
// Dependencies: {}
impl std :: fmt :: Display for ValueRange { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { ok ! (self . start_inclusive . fmt (f)) ; if self . is_fixed () { } else if self . end_inclusive == usize :: MAX { ok ! (".." . fmt (f)) ; } else { ok ! ("..=" . fmt (f)) ; ok ! (self . end_inclusive . fmt (f)) ; } Ok (()) } }
};
}
