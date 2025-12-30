// Generated macro for impl_2785 (impl)
macro_rules! Depcrate_timingimpl_2785 {
() => {
// Module: crate::timing
// Provides: {"impl_2785"}
// Dependencies: {}
impl Pass { fn idx (self) -> usize { self as usize } # [doc = " Description of the pass."] pub fn description (self) -> & 'static str { match DESCRIPTIONS . get (self . idx ()) { Some (s) => s , None => "<no pass>" , } } }
};
}
