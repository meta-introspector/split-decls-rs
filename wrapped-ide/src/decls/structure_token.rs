macro_rules! deps {
    () => {
        StructureNodeKind!();
        StructureNode!();
    };
}

macro_rules! structure_token {
    () => {
        deps!();
        fn structure_token (token : SyntaxToken) -> Option < StructureNode > { if let Some (comment) = ast :: Comment :: cast (token) { let text = comment . text () . trim () ; if let Some (region_name) = text . strip_prefix ("// region:") . map (str :: trim) . filter (| it | ! it . is_empty ()) { return Some (StructureNode { parent : None , label : region_name . to_owned () , navigation_range : comment . syntax () . text_range () , node_range : comment . syntax () . text_range () , kind : StructureNodeKind :: Region , detail : None , deprecated : false , }) ; } } None }
    };
}

structure_token!()