macro_rules! deps {
    () => {
        Ranker!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl < 'a > Ranker < 'a > { pub const MAX_RANK : usize = 0b1110 ; pub fn from_token (token : & 'a syntax :: SyntaxToken) -> Self { let kind = token . kind () ; Ranker { kind , text : token . text () , ident_kind : kind . is_any_identifier () } } # [doc = " A utility function that ranks a token again a given kind and text, returning a number that"] # [doc = " represents how close the token is to the given kind and text."] pub fn rank_token (& self , tok : & syntax :: SyntaxToken) -> usize { let tok_kind = tok . kind () ; let exact_same_kind = tok_kind == self . kind ; let both_idents = exact_same_kind || (tok_kind . is_any_identifier () && self . ident_kind) ; let same_text = tok . text () == self . text ; let no_tt_parent = tok . parent () . is_some_and (| it | it . kind () != parser :: SyntaxKind :: TOKEN_TREE) ; (both_idents as usize) | ((exact_same_kind as usize) << 1) | ((same_text as usize) << 2) | ((no_tt_parent as usize) << 3) } }
    };
}

impl_263!();