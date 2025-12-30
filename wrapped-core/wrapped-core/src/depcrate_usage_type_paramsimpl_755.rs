// Generated macro for impl_755 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_755 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_755"}
// Dependencies: {}
impl < 'i , T , I > CollectTypeParams for T where T : IntoIterator < Item = & 'i I > , I : 'i + UsesTypeParams , { fn collect_type_params < 'a > (self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { self . into_iter () . fold (IdentRefSet :: with_capacity_and_hasher (type_set . len () , Default :: default ()) , | state , value | union_in_place (state , value . uses_type_params (options , type_set)) ,) } fn collect_type_params_cloned (self , options : & Options , type_set : & IdentSet) -> IdentSet { self . collect_type_params (options , type_set) . into_iter () . cloned () . collect () } }
};
}
