// Generated macro for macro_6996 (macro)
macro_rules! Depcrate_methodsmacro_6996 {
() => {
// Module: crate::methods
// Provides: {"macro_6996"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.get().unwrap()` (or"] # [doc = " `.get_mut().unwrap`) on a standard library type which implements `Index`"] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Using the Index trait (`[]`) is more clear and more"] # [doc = " concise."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Not a replacement for error handling: Using either"] # [doc = " `.unwrap()` or the Index trait (`[]`) carries the risk of causing a `panic`"] # [doc = " if the value being accessed is `None`. If the use of `.get().unwrap()` is a"] # [doc = " temporary placeholder for dealing with the `Option` type, then this does"] # [doc = " not mitigate the need for error handling. If there is a chance that `.get()`"] # [doc = " will be `None` in your program, then it is advisable that the `None` case"] # [doc = " is handled in a future refactor instead of using `.unwrap()` or the Index"] # [doc = " trait."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut some_vec = vec![0, 1, 2, 3];"] # [doc = " let last = some_vec.get(3).unwrap();"] # [doc = " *some_vec.get_mut(0).unwrap() = 1;"] # [doc = " ```"] # [doc = " The correct use would be:"] # [doc = " ```no_run"] # [doc = " let mut some_vec = vec![0, 1, 2, 3];"] # [doc = " let last = some_vec[3];"] # [doc = " some_vec[0] = 1;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub GET_UNWRAP , restriction , "using `.get().unwrap()` or `.get_mut().unwrap()` when using `[]` would work instead" }
};
}
