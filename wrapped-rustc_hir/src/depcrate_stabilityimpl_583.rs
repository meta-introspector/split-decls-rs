// Generated macro for impl_583 (impl)
macro_rules! Depcrate_stabilityimpl_583 {
() => {
// Module: crate::stability
// Provides: {"impl_583"}
// Dependencies: {}
impl StabilityLevel { pub fn is_unstable (& self) -> bool { matches ! (self , StabilityLevel :: Unstable { .. }) } pub fn is_stable (& self) -> bool { matches ! (self , StabilityLevel :: Stable { .. }) } pub fn stable_since (& self) -> Option < StableSince > { match * self { StabilityLevel :: Stable { since , .. } => Some (since) , StabilityLevel :: Unstable { .. } => None , } } }
};
}
