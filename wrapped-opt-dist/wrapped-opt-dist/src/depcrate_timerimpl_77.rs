// Generated macro for impl_77 (impl)
macro_rules! Depcrate_timerimpl_77 {
() => {
// Module: crate::timer
// Provides: {"impl_77"}
// Dependencies: {}
impl SectionEntry { fn total_duration (& self) -> Duration { match self { SectionEntry :: Duration (duration) => * duration , SectionEntry :: SubSection (timer) => timer . total_duration () , } } }
};
}
