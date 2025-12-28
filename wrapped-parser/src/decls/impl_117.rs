macro_rules! deps {
    () => {
        Reparser!();
        Parser!();
        Output!();
        Input!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl Reparser { # [doc = " If the node is a braced block, return the corresponding `Reparser`."] pub fn for_node (node : SyntaxKind , first_child : Option < SyntaxKind > , parent : Option < SyntaxKind > ,) -> Option < Reparser > { grammar :: reparser (node , first_child , parent) . map (Reparser) } # [doc = " Re-parse given tokens using this `Reparser`."] # [doc = ""] # [doc = " Tokens must start with `{`, end with `}` and form a valid brace"] # [doc = " sequence."] pub fn parse (self , tokens : & Input , edition : Edition) -> Output { let Reparser (r) = self ; let mut p = parser :: Parser :: new (tokens , edition) ; r (& mut p) ; let events = p . finish () ; event :: process (events) } }
    };
}

impl_117!();