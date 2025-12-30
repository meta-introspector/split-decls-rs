// Generated macro for MTLDevicePrivate (trait)
macro_rules! Depcrate_privateMTLDevicePrivate {
() => {
// Module: crate::private
// Provides: {"MTLDevicePrivate"}
// Dependencies: {}
pub unsafe trait MTLDevicePrivate : Message { unsafe fn vendorName (& self) -> Retained < objc2_foundation :: NSString > { unsafe { msg_send ! [self , vendorName] } } unsafe fn familyName (& self) -> Retained < objc2_foundation :: NSString > { unsafe { msg_send ! [self , familyName] } } }
};
}
