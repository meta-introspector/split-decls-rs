macro_rules! LocalSource {
    () => {
        # [doc = " Hints at the original code for a let statement."] # [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum LocalSource { # [doc = " A `match _ { .. }`."] Normal , # [doc = " When lowering async functions, we create locals within the `async move` so that"] # [doc = " all parameters are dropped after the future is polled."] # [doc = ""] # [doc = " ```ignore (pseudo-Rust)"] # [doc = " async fn foo(<pattern> @ x: Type) {"] # [doc = "     async move {"] # [doc = "         let <pattern> = x;"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] AsyncFn , # [doc = " A desugared `<expr>.await`."] AwaitDesugar , # [doc = " A desugared `expr = expr`, where the LHS is a tuple, struct, array or underscore expression."] # [doc = " The span is that of the `=` sign."] AssignDesugar (Span) , # [doc = " A contract `#[ensures(..)]` attribute injects a let binding for the check that runs at point of return."] Contract , }
    };
}

LocalSource!()