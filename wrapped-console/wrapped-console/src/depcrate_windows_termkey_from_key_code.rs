// Generated macro for key_from_key_code (function)
macro_rules! Depcrate_windows_termkey_from_key_code {
() => {
// Module: crate::windows_term
// Provides: {"key_from_key_code"}
// Dependencies: {}
pub (crate) fn key_from_key_code (code : VIRTUAL_KEY) -> Key { use windows_sys :: Win32 :: UI :: Input :: KeyboardAndMouse ; match code { KeyboardAndMouse :: VK_LEFT => Key :: ArrowLeft , KeyboardAndMouse :: VK_RIGHT => Key :: ArrowRight , KeyboardAndMouse :: VK_UP => Key :: ArrowUp , KeyboardAndMouse :: VK_DOWN => Key :: ArrowDown , KeyboardAndMouse :: VK_RETURN => Key :: Enter , KeyboardAndMouse :: VK_ESCAPE => Key :: Escape , KeyboardAndMouse :: VK_BACK => Key :: Backspace , KeyboardAndMouse :: VK_TAB => Key :: Tab , KeyboardAndMouse :: VK_HOME => Key :: Home , KeyboardAndMouse :: VK_END => Key :: End , KeyboardAndMouse :: VK_DELETE => Key :: Del , KeyboardAndMouse :: VK_SHIFT => Key :: Shift , KeyboardAndMouse :: VK_MENU => Key :: Alt , _ => Key :: Unknown , } }
};
}
