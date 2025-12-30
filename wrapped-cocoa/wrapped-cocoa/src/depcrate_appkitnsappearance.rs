// Generated macro for NSAppearance (function)
macro_rules! Depcrate_appkitNSAppearance {
() => {
// Module: crate::appkit
// Provides: {"NSAppearance"}
// Dependencies: {}
pub unsafe fn NSAppearance (named : id) -> id { objc :: msg_send ! [class ! (NSAppearance) , appearanceNamed : named] }
};
}
