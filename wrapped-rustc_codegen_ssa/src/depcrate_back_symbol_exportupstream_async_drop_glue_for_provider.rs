// Generated macro for upstream_async_drop_glue_for_provider (function)
macro_rules! Depcrate_back_symbol_exportupstream_async_drop_glue_for_provider {
() => {
// Module: crate::back::symbol_export
// Provides: {"upstream_async_drop_glue_for_provider"}
// Dependencies: {}
fn upstream_async_drop_glue_for_provider < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> Option < CrateNum > { let def_id = tcx . lang_items () . async_drop_in_place_fn () ? ; tcx . upstream_monomorphizations_for (def_id) ? . get (& args) . cloned () }
};
}
