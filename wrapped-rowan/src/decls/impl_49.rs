macro_rules! deps {
    () => {
        WalkEvent!();
        PreorderWithTokens!();
        SyntaxNode!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl PreorderWithTokens { fn new (start : SyntaxNode) -> PreorderWithTokens { let next = Some (WalkEvent :: Enter (start . clone () . into ())) ; PreorderWithTokens { start : start . into () , next , skip_subtree : false } } pub fn skip_subtree (& mut self) { self . skip_subtree = true ; } # [cold] fn do_skip (& mut self) { self . next = self . next . take () . map (| next | match next { WalkEvent :: Enter (first_child) => WalkEvent :: Leave (first_child . parent () . unwrap () . into ()) , WalkEvent :: Leave (parent) => WalkEvent :: Leave (parent) , }) } }
    };
}

impl_49!();