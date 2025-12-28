macro_rules! deps {
    () => {
        SyntaxNode!();
        SyntaxNodeChildren!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl Iterator for SyntaxNodeChildren { type Item = SyntaxNode ; fn next (& mut self) -> Option < SyntaxNode > { if ! self . next_initialized { self . next = self . parent . first_child () ; self . next_initialized = true ; } else { self . next = self . next . take () . and_then (| next | next . to_next_sibling ()) ; } self . next . clone () } }
    };
}

impl_37!()