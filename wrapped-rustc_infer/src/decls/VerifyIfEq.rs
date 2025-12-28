macro_rules! deps {
    () => {
        VerifyBound!();
    };
}

macro_rules! VerifyIfEq {
    () => {
        deps!();
        # [doc = " This is a \"conditional bound\" that checks the result of inference"] # [doc = " and supplies a bound if it ended up being relevant. It's used in situations"] # [doc = " like this:"] # [doc = ""] # [doc = " ```rust,ignore (pseudo-Rust)"] # [doc = " fn foo<'a, 'b, T: SomeTrait<'a>>"] # [doc = " where"] # [doc = "    <T as SomeTrait<'a>>::Item: 'b"] # [doc = " ```"] # [doc = ""] # [doc = " If we have an obligation like `<T as SomeTrait<'?x>>::Item: 'c`, then"] # [doc = " we don't know yet whether it suffices to show that `'b: 'c`. If `'?x` winds"] # [doc = " up being equal to `'a`, then the where-clauses on function applies, and"] # [doc = " in that case we can show `'b: 'c`. But if `'?x` winds up being something"] # [doc = " else, the bound isn't relevant."] # [doc = ""] # [doc = " In the [`VerifyBound`], this struct is enclosed in `Binder` to account"] # [doc = " for cases like"] # [doc = ""] # [doc = " ```rust,ignore (pseudo-Rust)"] # [doc = " where for<'a> <T as SomeTrait<'a>::Item: 'a"] # [doc = " ```"] # [doc = ""] # [doc = " The idea is that we have to find some instantiation of `'a` that can"] # [doc = " make `<T as SomeTrait<'a>>::Item` equal to the final value of `G`,"] # [doc = " the generic we are checking."] # [doc = ""] # [doc = " ```ignore (pseudo-rust)"] # [doc = " fn(min) -> bool {"] # [doc = "     exists<'a> {"] # [doc = "         if G == K {"] # [doc = "             B(min)"] # [doc = "         } else {"] # [doc = "             false"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [derive (Debug , Copy , Clone , TypeFoldable , TypeVisitable)] pub struct VerifyIfEq < 'tcx > { # [doc = " Type which must match the generic `G`"] pub ty : Ty < 'tcx > , # [doc = " Bound that applies if `ty` is equal."] pub bound : Region < 'tcx > , }
    };
}

VerifyIfEq!()