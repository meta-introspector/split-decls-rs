macro_rules! deps {
    () => {
        ResolvedRule!();
        ResolutionScope!();
        Resolver!();
        SsrError!();
        ParsedRule!();
        Placeholder!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'db > ResolvedRule < 'db > { pub (crate) fn new (rule : parsing :: ParsedRule , resolution_scope : & ResolutionScope < 'db > , index : usize ,) -> Result < ResolvedRule < 'db > , SsrError > { hir :: attach_db (resolution_scope . scope . db , | | { let resolver = Resolver { resolution_scope , placeholders_by_stand_in : rule . placeholders_by_stand_in , } ; let resolved_template = match rule . template { Some (template) => Some (resolver . resolve_pattern_tree (template) ?) , None => None , } ; Ok (ResolvedRule { pattern : resolver . resolve_pattern_tree (rule . pattern) ? , template : resolved_template , index , }) }) } pub (crate) fn get_placeholder (& self , token : & SyntaxToken) -> Option < & Placeholder > { if token . kind () != SyntaxKind :: IDENT { return None ; } self . pattern . placeholders_by_stand_in . get (token . text ()) } }
    };
}

impl_79!()