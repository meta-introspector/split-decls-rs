// Generated macro for macro_4473 (macro)
macro_rules! Depcrate_map_unit_fnmacro_4473 {
() => {
// Module: crate::map_unit_fn
// Provides: {"macro_4473"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `option.map(f)` where f is a function"] # [doc = " or closure that returns the unit type `()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more clearly with"] # [doc = " an if let statement"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn do_stuff() -> Option<String> { Some(String::new()) }"] # [doc = " # fn log_err_msg(foo: String) -> Option<String> { Some(foo) }"] # [doc = " # fn format_msg(foo: String) -> String { String::new() }"] # [doc = " let x: Option<String> = do_stuff();"] # [doc = " x.map(log_err_msg);"] # [doc = " # let x: Option<String> = do_stuff();"] # [doc = " x.map(|msg| log_err_msg(format_msg(msg)));"] # [doc = " ```"] # [doc = ""] # [doc = " The correct use would be:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn do_stuff() -> Option<String> { Some(String::new()) }"] # [doc = " # fn log_err_msg(foo: String) -> Option<String> { Some(foo) }"] # [doc = " # fn format_msg(foo: String) -> String { String::new() }"] # [doc = " let x: Option<String> = do_stuff();"] # [doc = " if let Some(msg) = x {"] # [doc = "     log_err_msg(msg);"] # [doc = " }"] # [doc = ""] # [doc = " # let x: Option<String> = do_stuff();"] # [doc = " if let Some(msg) = x {"] # [doc = "     log_err_msg(format_msg(msg));"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub OPTION_MAP_UNIT_FN , complexity , "using `option.map(f)`, where `f` is a function or closure that returns `()`" }
};
}
