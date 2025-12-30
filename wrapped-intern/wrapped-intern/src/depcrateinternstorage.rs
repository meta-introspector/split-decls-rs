// Generated macro for InternStorage (struct)
macro_rules! DepcrateInternStorage {
() => {
// Module: crate
// Provides: {"InternStorage"}
// Dependencies: {}
pub struct InternStorage < T : ? Sized > { map : OnceLock < InternMap < T > > , }
};
}
