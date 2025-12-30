// Generated macro for NSUserDefaults (trait)
macro_rules! Depcrate_foundationNSUserDefaults {
() => {
// Module: crate::foundation
// Provides: {"NSUserDefaults"}
// Dependencies: {}
pub trait NSUserDefaults { unsafe fn standardUserDefaults () -> Self ; unsafe fn setBool_forKey_ (self , value : BOOL , key : id) ; unsafe fn bool_forKey_ (self , key : id) -> BOOL ; unsafe fn removeObject_forKey_ (self , key : id) ; }
};
}
