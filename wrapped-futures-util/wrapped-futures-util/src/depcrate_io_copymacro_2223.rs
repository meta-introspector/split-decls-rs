// Generated macro for macro_2223 (macro)
macro_rules! Depcrate_io_copymacro_2223 {
() => {
// Module: crate::io::copy
// Provides: {"macro_2223"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`copy()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Copy <'a , R , W : ? Sized > { # [pin] inner : CopyBuf <'a , BufReader < R >, W >, } }
};
}
