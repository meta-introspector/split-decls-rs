// Generated macro for Context (struct)
macro_rules! Depcrate_pack_explodeContext {
() => {
// Module: crate::pack::explode
// Provides: {"Context"}
// Dependencies: {}
# [derive (Default)] pub struct Context { pub thread_limit : Option < usize > , pub delete_pack : bool , pub sink_compress : bool , pub verify : bool , pub should_interrupt : Arc < AtomicBool > , pub object_hash : gix :: hash :: Kind , }
};
}
