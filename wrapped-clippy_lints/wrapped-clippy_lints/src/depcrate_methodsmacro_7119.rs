// Generated macro for macro_7119 (macro)
macro_rules! Depcrate_methodsmacro_7119 {
() => {
// Module: crate::methods
// Provides: {"macro_7119"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.and_then(|x| Some(y))`, `_.and_then(|x| Ok(y))`"] # [doc = " or `_.or_else(|x| Err(y))`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can be written more concisely as `_.map(|x| y)` or `_.map_err(|x| y)`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn opt() -> Option<&'static str> { Some(\"42\") }"] # [doc = " # fn res() -> Result<&'static str, &'static str> { Ok(\"42\") }"] # [doc = " let _ = opt().and_then(|s| Some(s.len()));"] # [doc = " let _ = res().and_then(|s| if s.len() == 42 { Ok(10) } else { Ok(20) });"] # [doc = " let _ = res().or_else(|s| if s.len() == 42 { Err(10) } else { Err(20) });"] # [doc = " ```"] # [doc = ""] # [doc = " The correct use would be:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn opt() -> Option<&'static str> { Some(\"42\") }"] # [doc = " # fn res() -> Result<&'static str, &'static str> { Ok(\"42\") }"] # [doc = " let _ = opt().map(|s| s.len());"] # [doc = " let _ = res().map(|s| if s.len() == 42 { 10 } else { 20 });"] # [doc = " let _ = res().map_err(|s| if s.len() == 42 { 10 } else { 20 });"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub BIND_INSTEAD_OF_MAP , complexity , "using `Option.and_then(|x| Some(y))`, which is more succinctly expressed as `map(|x| y)`" }
};
}
