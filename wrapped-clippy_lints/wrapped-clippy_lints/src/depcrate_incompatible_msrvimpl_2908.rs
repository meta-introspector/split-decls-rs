// Generated macro for impl_2908 (impl)
macro_rules! Depcrate_incompatible_msrvimpl_2908 {
() => {
// Module: crate::incompatible_msrv
// Provides: {"impl_2908"}
// Dependencies: {}
impl StdCrates { fn new (tcx : TyCtxt < '_ >) -> Self { let mut res = Self ([None ; _]) ; for & krate in tcx . crates (()) { match tcx . crate_name (krate) { sym :: alloc => res . 0 [0] = Some (krate) , sym :: core => res . 0 [1] = Some (krate) , sym :: core_arch => res . 0 [2] = Some (krate) , sym :: proc_macro => res . 0 [3] = Some (krate) , sym :: std => res . 0 [4] = Some (krate) , sym :: std_detect => res . 0 [5] = Some (krate) , _ => { } , } } res } fn contains (& self , krate : CrateNum) -> bool { self . 0 . contains (& Some (krate)) } }
};
}
