macro_rules! deps {
    () => {
        PathKind!();
    };
}

macro_rules! validate_snippet {
    () => {
        deps!();
        fn validate_snippet (snippet : & [String] , description : & str , requires : & [String] ,) -> Option < (Box < [ModPath] > , String , Option < Box < str > >) > { let mut imports = Vec :: with_capacity (requires . len ()) ; for path in requires . iter () { let use_path = ModPath :: from_segments (hir :: PathKind :: Plain , path . split ("::") . map (Symbol :: intern) . map (Name :: new_symbol_root) ,) ; imports . push (use_path) ; } let snippet = snippet . iter () . join ("\n") ; let description = (! description . is_empty ()) . then (| | description . split_once ('\n') . map_or (description , | (it , _) | it)) . map (ToOwned :: to_owned) . map (Into :: into) ; Some ((imports . into_boxed_slice () , snippet , description)) }
    };
}

validate_snippet!();