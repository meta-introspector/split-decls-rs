macro_rules! deps {
    () => {
        Preorder!();
        SyntaxNode!();
        WalkEvent!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Preorder { fn new (start : SyntaxNode) -> Preorder { let next = Some (WalkEvent :: Enter (start . clone ())) ; Preorder { start , next , skip_subtree : false } } pub fn skip_subtree (& mut self) { self . skip_subtree = true ; } # [cold] fn do_skip (& mut self) { self . next = self . next . take () . map (| next | match next { WalkEvent :: Enter (first_child) => WalkEvent :: Leave (first_child . parent () . unwrap ()) , WalkEvent :: Leave (parent) => WalkEvent :: Leave (parent) , }) } }
    };
}

impl_46!()