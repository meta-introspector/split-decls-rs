macro_rules! deps {
    () => {
        NodeOrToken!();
        PreorderWithTokens!();
        SyntaxElement!();
        WalkEvent!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Iterator for PreorderWithTokens { type Item = WalkEvent < SyntaxElement > ; fn next (& mut self) -> Option < WalkEvent < SyntaxElement > > { if self . skip_subtree { self . do_skip () ; self . skip_subtree = false ; } let next = self . next . take () ; self . next = next . as_ref () . and_then (| next | { Some (match next { WalkEvent :: Enter (el) => match el { NodeOrToken :: Node (node) => match node . first_child_or_token () { Some (child) => WalkEvent :: Enter (child) , None => WalkEvent :: Leave (node . clone () . into ()) , } , NodeOrToken :: Token (token) => WalkEvent :: Leave (token . clone () . into ()) , } , WalkEvent :: Leave (el) if el == & self . start => return None , WalkEvent :: Leave (el) => match el . next_sibling_or_token () { Some (sibling) => WalkEvent :: Enter (sibling) , None => WalkEvent :: Leave (el . parent () ? . into ()) , } , }) }) ; next } }
    };
}

impl_50!();