// Generated macro for impl_1812 (impl)
macro_rules! Depcrate_user_notificationimpl_1812 {
() => {
// Module: crate::user_notification
// Provides: {"impl_1812"}
// Dependencies: {}
impl CFUserNotification { # [doc (alias = "CFUserNotificationCheckBoxChecked")] pub extern "C" fn check_box_checked (i : CFIndex) -> CFOptionFlags { (1usize << (8 + i)) as CFOptionFlags } # [doc (alias = "CFUserNotificationSecureTextField")] pub extern "C" fn secure_text_field (i : CFIndex) -> CFOptionFlags { (1usize << (16 + i)) as CFOptionFlags } # [doc (alias = "CFUserNotificationPopUpSelection")] pub fn pop_up_selection (n : CFIndex) -> CFOptionFlags { (n << 24) as CFOptionFlags } }
};
}
