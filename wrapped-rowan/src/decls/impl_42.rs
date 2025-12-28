macro_rules! deps {
    () => {
        SyntaxElementChildren!();
        SyntaxElement!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Iterator for SyntaxElementChildren { type Item = SyntaxElement ; fn next (& mut self) -> Option < SyntaxElement > { if ! self . next_initialized { self . next = self . parent . first_child_or_token () ; self . next_initialized = true ; } else { self . next = self . next . take () . and_then (| next | next . to_next_sibling_or_token ()) ; } self . next . clone () } }
    };
}

impl_42!();