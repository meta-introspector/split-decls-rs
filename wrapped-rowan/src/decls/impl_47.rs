macro_rules! deps {
    () => {
        Preorder!();
        SyntaxNode!();
        WalkEvent!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Iterator for Preorder { type Item = WalkEvent < SyntaxNode > ; fn next (& mut self) -> Option < WalkEvent < SyntaxNode > > { if self . skip_subtree { self . do_skip () ; self . skip_subtree = false ; } let next = self . next . take () ; self . next = next . as_ref () . and_then (| next | { Some (match next { WalkEvent :: Enter (node) => match node . first_child () { Some (child) => WalkEvent :: Enter (child) , None => WalkEvent :: Leave (node . clone ()) , } , WalkEvent :: Leave (node) => { if node == & self . start { return None ; } match node . next_sibling () { Some (sibling) => WalkEvent :: Enter (sibling) , None => WalkEvent :: Leave (node . parent () ?) , } } }) }) ; next } }
    };
}

impl_47!();