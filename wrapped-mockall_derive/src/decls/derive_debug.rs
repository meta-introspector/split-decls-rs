macro_rules! derive_debug {
    () => {
        # [doc = " Generate a #[derive(Debug)] Attribute"] fn derive_debug () -> Attribute { let ml = parse2 (quote ! (derive (Debug))) . unwrap () ; Attribute { pound_token : < Token ! [#] > :: default () , style : AttrStyle :: Outer , bracket_token : token :: Bracket :: default () , meta : Meta :: List (ml) } }
    };
}

derive_debug!();