// Generated macro for Context (struct)
macro_rules! Depcrate_pack_receiveContext {
() => {
// Module: crate::pack::receive
// Provides: {"Context"}
// Dependencies: {}
pub struct Context < W > { pub thread_limit : Option < usize > , pub format : OutputFormat , pub should_interrupt : Arc < AtomicBool > , pub out : W , pub object_hash : gix :: hash :: Kind , }
};
}
