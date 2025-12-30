// Generated macro for securely (function)
macro_rules! Depcratesecurely {
() => {
// Module: crate
// Provides: {"securely"}
// Dependencies: {}
# [doc = " Ask for information _securely_ after showing the `prompt` (like `\"password: \"`) by not showing what's typed."] # [doc = ""] # [doc = " Use [`ask()`] for more control."] pub fn securely (prompt : impl AsRef < str >) -> Result < String , Error > { imp :: ask (prompt . as_ref () , & Options { mode : Mode :: Hidden , askpass : None , } ,) }
};
}
