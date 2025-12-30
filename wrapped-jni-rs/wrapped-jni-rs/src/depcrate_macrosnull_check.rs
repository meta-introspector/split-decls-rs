// Generated macro for null_check (macro)
macro_rules! Depcrate_macrosnull_check {
() => {
// Module: crate::macros
// Provides: {"null_check"}
// Dependencies: {}
# [doc = " Maps a pointer to either Ok(ptr) or Err(Error::NullPtr)"] # [doc = ""] # [doc = " This makes it reasonably ergonomic to use `?` to early-exit with an `Err` in"] # [doc = " case of `null` pointer arguments."] # [doc = ""] # [doc = " Unlike earlier macros this avoids using `return`, since that can result in"] # [doc = " surprising control flow if the caller doesn't realize that a macro might"] # [doc = " explicitly return from the current function."] macro_rules ! null_check { ($ obj : expr , $ ctx : expr) => { if $ obj . is_null () { Err ($ crate :: errors :: Error :: NullPtr ($ ctx)) } else { Ok ($ obj) } } ; }
};
}
