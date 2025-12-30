// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < T , G : TearDown > Drop for Fixture < T , G > { fn drop (& mut self) { self . guard . take () . map (| g | g . tear_down ()) ; } }
};
}
