macro_rules! deps {
    () => {
        Pattern!();
        Error!();
        Spec!();
    };
}

macro_rules! mapping_from_pattern {
    () => {
        deps!();
        # [doc = " Create a new specification to support matches from `pathspec`, [normalizing](Pattern::normalize()) it with `prefix` and `root`."] fn mapping_from_pattern (mut pathspec : Pattern , prefix : & Path , root : & Path , sequence_number : usize ,) -> Result < gix_glob :: search :: pattern :: Mapping < Spec > , crate :: normalize :: Error > { pathspec . normalize (prefix , root) ? ; let mut match_all = pathspec . is_nil () ; let glob = { let mut g = gix_glob :: Pattern :: from_bytes_without_negation (& pathspec . path) . unwrap_or_else (| | { match_all = true ; gix_glob :: Pattern { text : pathspec . path . clone () , mode : gix_glob :: pattern :: Mode :: empty () , first_wildcard_pos : None , } }) ; g . mode |= gix_glob :: pattern :: Mode :: ABSOLUTE ; if pathspec . signature . contains (MagicSignature :: MUST_BE_DIR) { g . mode |= gix_glob :: pattern :: Mode :: MUST_BE_DIR ; } g } ; Ok (gix_glob :: search :: pattern :: Mapping { pattern : glob , value : Spec { attrs_match : { (! pathspec . attributes . is_empty ()) . then (| | { let mut out = gix_attributes :: search :: Outcome :: default () ; out . initialize_with_selection (& Default :: default () , pathspec . attributes . iter () . map (| a | a . name . as_str ()) ,) ; out }) } , pattern : pathspec , } , sequence_number , }) }
    };
}

mapping_from_pattern!()