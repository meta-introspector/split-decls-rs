macro_rules! deps {
    () => {
        NodeOrToken!();
        Language!();
        SyntaxNode!();
        SyntaxElement!();
        SyntaxToken!();
        Direction!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < L : Language > SyntaxToken < L > { # [doc = " Returns a green tree, equal to the green tree this token"] # [doc = " belongs to, except with this token substituted. The complexity"] # [doc = " of the operation is proportional to the depth of the tree."] pub fn replace_with (& self , new_token : GreenToken) -> GreenNode { self . raw . replace_with (new_token) } pub fn kind (& self) -> L :: Kind { L :: kind_from_raw (self . raw . kind ()) } pub fn text_range (& self) -> TextRange { self . raw . text_range () } pub fn index (& self) -> usize { self . raw . index () } pub fn text (& self) -> & str { self . raw . text () } pub fn green (& self) -> & GreenTokenData { self . raw . green () } pub fn parent (& self) -> Option < SyntaxNode < L > > { self . raw . parent () . map (SyntaxNode :: from) } # [doc = " Iterator over all the ancestors of this token excluding itself."] # [deprecated = "use `SyntaxToken::parent_ancestors` instead"] pub fn ancestors (& self) -> impl Iterator < Item = SyntaxNode < L > > + use < L > { self . parent_ancestors () } # [doc = " Iterator over all the ancestors of this token excluding itself."] pub fn parent_ancestors (& self) -> impl Iterator < Item = SyntaxNode < L > > + use < L > { self . raw . ancestors () . map (SyntaxNode :: from) } pub fn next_sibling_or_token (& self) -> Option < SyntaxElement < L > > { self . raw . next_sibling_or_token () . map (NodeOrToken :: from) } pub fn prev_sibling_or_token (& self) -> Option < SyntaxElement < L > > { self . raw . prev_sibling_or_token () . map (NodeOrToken :: from) } pub fn siblings_with_tokens (& self , direction : Direction ,) -> impl Iterator < Item = SyntaxElement < L > > + use < L > { self . raw . siblings_with_tokens (direction) . map (SyntaxElement :: from) } # [doc = " Next token in the tree (i.e, not necessary a sibling)."] pub fn next_token (& self) -> Option < SyntaxToken < L > > { self . raw . next_token () . map (SyntaxToken :: from) } # [doc = " Previous token in the tree (i.e, not necessary a sibling)."] pub fn prev_token (& self) -> Option < SyntaxToken < L > > { self . raw . prev_token () . map (SyntaxToken :: from) } pub fn detach (& self) { self . raw . detach () } }
    };
}

impl_63!()