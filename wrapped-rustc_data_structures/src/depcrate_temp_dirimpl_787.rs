// Generated macro for impl_787 (impl)
macro_rules! Depcrate_temp_dirimpl_787 {
() => {
// Module: crate::temp_dir
// Provides: {"impl_787"}
// Dependencies: {}
impl MaybeTempDir { pub fn new (dir : TempDir , keep_on_drop : bool) -> MaybeTempDir { MaybeTempDir { dir : ManuallyDrop :: new (dir) , keep : keep_on_drop } } }
};
}
