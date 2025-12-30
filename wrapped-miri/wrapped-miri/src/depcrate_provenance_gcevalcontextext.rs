// Generated macro for EvalContextExt (trait)
macro_rules! Depcrate_provenance_gcEvalContextExt {
() => {
// Module: crate::provenance_gc
// Provides: {"EvalContextExt"}
// Dependencies: {}
pub trait EvalContextExt < 'tcx > : MiriInterpCxExt < 'tcx > { fn run_provenance_gc (& mut self) { let this = self . eval_context_mut () ; let mut tags = FxHashSet :: default () ; let mut alloc_ids = FxHashSet :: default () ; this . visit_provenance (& mut | id , tag | { if let Some (id) = id { alloc_ids . insert (id) ; } if let Some (tag) = tag { tags . insert (tag) ; } }) ; remove_unreachable_tags (this , tags) ; remove_unreachable_allocs (this , alloc_ids) ; } }
};
}
