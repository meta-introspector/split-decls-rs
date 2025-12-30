// Generated macro for openly (function)
macro_rules! Depcrateopenly {
() => {
// Module: crate
// Provides: {"openly"}
// Dependencies: {}
# [doc = " Ask for information typed by the user into the terminal after showing the prompt, like `\"Username: `."] # [doc = ""] # [doc = " Use [`ask()`] for more control."] pub fn openly (prompt : impl AsRef < str >) -> Result < String , Error > { imp :: ask (prompt . as_ref () , & Options { mode : Mode :: Visible , askpass : None , } ,) }
};
}
