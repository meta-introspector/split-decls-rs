macro_rules! YieldSource {
    () => {
        # [doc = " The yield kind that caused an `ExprKind::Yield`."] # [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum YieldSource { # [doc = " An `<expr>.await`."] Await { expr : Option < HirId > } , # [doc = " A plain `yield`."] Yield , }
    };
}

YieldSource!()