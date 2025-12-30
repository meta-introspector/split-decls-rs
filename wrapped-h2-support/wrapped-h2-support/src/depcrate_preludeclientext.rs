// Generated macro for ClientExt (trait)
macro_rules! Depcrate_preludeClientExt {
() => {
// Module: crate::prelude
// Provides: {"ClientExt"}
// Dependencies: {}
pub trait ClientExt { fn run < 'a , F : Future + Unpin + 'a > (& 'a mut self , f : F ,) -> Pin < Box < dyn Future < Output = F :: Output > + 'a > > ; }
};
}
