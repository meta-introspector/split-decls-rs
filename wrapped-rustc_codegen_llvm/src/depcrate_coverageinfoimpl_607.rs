// Generated macro for impl_607 (impl)
macro_rules! Depcrate_coverageinfoimpl_607 {
() => {
// Module: crate::coverageinfo
// Provides: {"impl_607"}
// Dependencies: {}
impl < 'll , 'tcx > CguCoverageContext < 'll , 'tcx > { pub (crate) fn new () -> Self { Self { pgo_func_name_var_map : Default :: default () , covfun_section_name : Default :: default () } } # [doc = " Returns the list of instances considered \"used\" in this CGU, as"] # [doc = " inferred from the keys of `pgo_func_name_var_map`."] pub (crate) fn instances_used (& self) -> Vec < Instance < 'tcx > > { self . pgo_func_name_var_map . borrow () . keys () . copied () . collect :: < Vec < _ > > () } }
};
}
