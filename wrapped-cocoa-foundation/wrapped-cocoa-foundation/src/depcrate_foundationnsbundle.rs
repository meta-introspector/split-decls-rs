// Generated macro for NSBundle (trait)
macro_rules! Depcrate_foundationNSBundle {
() => {
// Module: crate::foundation
// Provides: {"NSBundle"}
// Dependencies: {}
pub trait NSBundle : Sized { unsafe fn mainBundle () -> Self ; unsafe fn loadNibNamed_owner_topLevelObjects_ (self , name : id , owner : id , topLevelObjects : * mut id ,) -> BOOL ; unsafe fn bundleIdentifier (self) -> id ; unsafe fn resourcePath (self) -> id ; }
};
}
