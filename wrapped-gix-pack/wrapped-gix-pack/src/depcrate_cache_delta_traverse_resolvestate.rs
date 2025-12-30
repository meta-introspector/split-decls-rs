// Generated macro for State (struct)
macro_rules! Depcrate_cache_delta_traverse_resolveState {
() => {
// Module: crate::cache::delta::traverse::resolve
// Provides: {"State"}
// Dependencies: {}
pub (super) struct State < 'items , F , MBFN , T : Send > { pub delta_bytes : Vec < u8 > , pub fully_resolved_delta_bytes : Vec < u8 > , pub progress : Box < dyn Progress > , pub resolve : F , pub modify_base : MBFN , pub child_items : & 'items ItemSliceSync < 'items , Item < T > > , }
};
}
