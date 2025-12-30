// Generated macro for Context (struct)
macro_rules! Depcrate_pack_indexContext {
() => {
// Module: crate::pack::index
// Provides: {"Context"}
// Dependencies: {}
pub struct Context < 'a , W : io :: Write > { pub thread_limit : Option < usize > , pub iteration_mode : IterationMode , pub format : OutputFormat , pub should_interrupt : & 'a AtomicBool , pub out : W , pub object_hash : gix :: hash :: Kind , }
};
}
