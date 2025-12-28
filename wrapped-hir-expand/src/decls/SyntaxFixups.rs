macro_rules! deps {
    () => {
        SyntaxFixupUndoInfo!();
    };
}

macro_rules! SyntaxFixups {
    () => {
        deps!();
        # [doc = " The result of calculating fixes for a syntax node -- a bunch of changes"] # [doc = " (appending to and replacing nodes), the information that is needed to"] # [doc = " reverse those changes afterwards, and a token map."] # [derive (Debug , Default)] pub (crate) struct SyntaxFixups { pub (crate) append : FxHashMap < SyntaxElement , Vec < Leaf > > , pub (crate) remove : FxHashSet < SyntaxElement > , pub (crate) undo_info : SyntaxFixupUndoInfo , }
    };
}

SyntaxFixups!();