// Generated macro for Pooled (struct)
macro_rules! DepcratePooled {
() => {
// Module: crate
// Provides: {"Pooled"}
// Dependencies: {}
# [doc = " A value borrowed from the [`Pool`] that can be dereferenced to `T`."] # [derive (Debug)] pub struct Pooled < T : Default + Reuse + 'static > { inner : T , pool : & 'static QueueShard < T > , }
};
}
