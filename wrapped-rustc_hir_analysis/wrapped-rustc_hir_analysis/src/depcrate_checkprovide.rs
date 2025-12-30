// Generated macro for provide (function)
macro_rules! Depcrate_checkprovide {
() => {
// Module: crate::check
// Provides: {"provide"}
// Dependencies: {}
# [doc = " Adds query implementations to the [Providers] vtable, see [`rustc_middle::query`]"] pub (super) fn provide (providers : & mut Providers) { * providers = Providers { adt_destructor , adt_async_destructor , region_scope_tree , collect_return_position_impl_trait_in_trait_tys , compare_impl_item : compare_impl_item :: compare_impl_item , check_coroutine_obligations : check :: check_coroutine_obligations , check_potentially_region_dependent_goals : check :: check_potentially_region_dependent_goals , check_type_wf : wfcheck :: check_type_wf , check_well_formed : wfcheck :: check_well_formed , .. * providers } ; }
};
}
