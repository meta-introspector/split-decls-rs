macro_rules! deps {
    () => {
        GenericArgsParentheses!();
        GenericArg!();
        AssocItemConstraint!();
    };
}

macro_rules! GenericArgs {
    () => {
        deps!();
        # [doc = " The generic arguments and associated item constraints of a path segment."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct GenericArgs < 'hir > { # [doc = " The generic arguments for this path segment."] pub args : & 'hir [GenericArg < 'hir >] , # [doc = " The associated item constraints for this path segment."] pub constraints : & 'hir [AssocItemConstraint < 'hir >] , # [doc = " Whether the arguments were written in parenthesized form (e.g., `Fn(T) -> U`)."] # [doc = ""] # [doc = " This is required mostly for pretty-printing and diagnostics,"] # [doc = " but also for changing lifetime elision rules to be \"function-like\"."] pub parenthesized : GenericArgsParentheses , # [doc = " The span encompassing the arguments, constraints and the surrounding brackets (`<>` or `()`)."] # [doc = ""] # [doc = " For example:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = "       Foo<A, B, AssocTy = D>           Fn(T, U, V) -> W"] # [doc = "          ^^^^^^^^^^^^^^^^^^^             ^^^^^^^^^"] # [doc = " ```"] # [doc = ""] # [doc = " Note that this may be:"] # [doc = " - empty, if there are no generic brackets (but there may be hidden lifetimes)"] # [doc = " - dummy, if this was generated during desugaring"] pub span_ext : Span , }
    };
}

GenericArgs!();