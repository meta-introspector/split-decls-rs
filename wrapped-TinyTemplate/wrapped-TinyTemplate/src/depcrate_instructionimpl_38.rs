// Generated macro for impl_38 (impl)
macro_rules! Depcrate_instructionimpl_38 {
() => {
// Module: crate::instruction
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'template > Deref for PathStep < 'template > { type Target = str ; fn deref (& self) -> & Self :: Target { match self { PathStep :: Name (s) => s , PathStep :: Index (s , _) => s , } } }
};
}
