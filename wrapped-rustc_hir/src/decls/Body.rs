macro_rules! deps {
    () => {
        Param!();
        Expr!();
    };
}

macro_rules! Body {
    () => {
        deps!();
        # [doc = " The body of a function, closure, or constant value. In the case of"] # [doc = " a function, the body contains not only the function body itself"] # [doc = " (which is an expression), but also the argument patterns, since"] # [doc = " those are something that the caller doesn't really care about."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " fn foo((x, y): (u32, u32)) -> u32 {"] # [doc = "     x + y"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Here, the `Body` associated with `foo()` would contain:"] # [doc = ""] # [doc = " - an `params` array containing the `(x, y)` pattern"] # [doc = " - a `value` containing the `x + y` expression (maybe wrapped in a block)"] # [doc = " - `coroutine_kind` would be `None`"] # [doc = ""] # [doc = " All bodies have an **owner**, which can be accessed via the HIR"] # [doc = " map using `body_owner_def_id()`."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Body < 'hir > { pub params : & 'hir [Param < 'hir >] , pub value : & 'hir Expr < 'hir > , }
    };
}

Body!();