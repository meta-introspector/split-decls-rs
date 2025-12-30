// Generated macro for macro_7194 (macro)
macro_rules! Depcrate_methodsmacro_7194 {
() => {
// Module: crate::methods
// Provides: {"macro_7194"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the suspicious use of `OpenOptions::create()`"] # [doc = " without an explicit `OpenOptions::truncate()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `create()` alone will either create a new file or open an"] # [doc = " existing file. If the file already exists, it will be"] # [doc = " overwritten when written to, but the file will not be"] # [doc = " truncated by default."] # [doc = " If less data is written to the file"] # [doc = " than it already contains, the remainder of the file will"] # [doc = " remain unchanged, and the end of the file will contain old"] # [doc = " data."] # [doc = " In most cases, one should either use `create_new` to ensure"] # [doc = " the file is created from scratch, or ensure `truncate` is"] # [doc = " called so that the truncation behaviour is explicit. `truncate(true)`"] # [doc = " will ensure the file is entirely overwritten with new data, whereas"] # [doc = " `truncate(false)` will explicitly keep the default behavior."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,no_run"] # [doc = " use std::fs::OpenOptions;"] # [doc = ""] # [doc = " OpenOptions::new().create(true);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,no_run"] # [doc = " use std::fs::OpenOptions;"] # [doc = ""] # [doc = " OpenOptions::new().create(true).truncate(true);"] # [doc = " ```"] # [clippy :: version = "1.77.0"] pub SUSPICIOUS_OPEN_OPTIONS , suspicious , "suspicious combination of options for opening a file" }
};
}
