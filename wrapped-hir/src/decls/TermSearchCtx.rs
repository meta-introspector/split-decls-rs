macro_rules! deps {
    () => {
        TermSearchConfig!();
        SemanticsScope!();
        Type!();
        Semantics!();
    };
}

macro_rules! TermSearchCtx {
    () => {
        deps!();
        # [doc = " Context for the `term_search` function"] # [derive (Debug)] pub struct TermSearchCtx < 'db , DB : HirDatabase > { # [doc = " Semantics for the program"] pub sema : & 'db Semantics < 'db , DB > , # [doc = " Semantic scope, captures context for the term search"] pub scope : & 'db SemanticsScope < 'db > , # [doc = " Target / expected output type"] pub goal : Type < 'db > , # [doc = " Configuration for term search"] pub config : TermSearchConfig , }
    };
}

TermSearchCtx!();