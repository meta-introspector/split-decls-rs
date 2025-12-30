// Generated macro for impl_358 (impl)
macro_rules! Depcrate_groupbylazyimpl_358 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_358"}
// Dependencies: {}
impl < K , I , F > GroupInner < K , I , F > where I : Iterator , { # [doc = " Called when a group is dropped"] fn drop_group (& mut self , client : usize) { if self . dropped_group == ! 0 || client > self . dropped_group { self . dropped_group = client ; } } }
};
}
