macro_rules! MatchSource {
    () => {
        # [doc = " Hints at the original code for a `match _ { .. }`."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug , HashStable_Generic , Encodable , Decodable)] pub enum MatchSource { # [doc = " A `match _ { .. }`."] Normal , # [doc = " A `expr.match { .. }`."] Postfix , # [doc = " A desugared `for _ in _ { .. }` loop."] ForLoopDesugar , # [doc = " A desugared `?` operator."] TryDesugar (HirId) , # [doc = " A desugared `<expr>.await`."] AwaitDesugar , # [doc = " A desugared `format_args!()`."] FormatArgs , }
    };
}

MatchSource!()