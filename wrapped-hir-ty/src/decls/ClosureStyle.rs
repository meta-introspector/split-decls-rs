macro_rules! ClosureStyle {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ClosureStyle { # [doc = " `impl FnX(i32, i32) -> i32`, where `FnX` is the most special trait between `Fn`, `FnMut`, `FnOnce` that the"] # [doc = " closure implements. This is the default."] ImplFn , # [doc = " `|i32, i32| -> i32`"] RANotation , # [doc = " `{closure#14825}`, useful for some diagnostics (like type mismatch) and internal usage."] ClosureWithId , # [doc = " `{closure#14825}<i32, ()>`, useful for internal usage."] ClosureWithSubst , # [doc = " `…`, which is the `TYPE_HINT_TRUNCATION`"] Hide , }
    };
}

ClosureStyle!();