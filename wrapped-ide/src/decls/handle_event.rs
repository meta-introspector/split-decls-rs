macro_rules! deps {
    () => {
        InlayHintCtx!();
    };
}

macro_rules! handle_event {
    () => {
        deps!();
        fn handle_event (ctx : & mut InlayHintCtx , node : WalkEvent < SyntaxNode >) -> Option < SyntaxNode > { match node { WalkEvent :: Enter (node) => { if let Some (node) = ast :: AnyHasGenericParams :: cast (node . clone ()) { let params = node . generic_param_list () . map (| it | { it . lifetime_params () . filter_map (| it | { it . lifetime () . map (| it | format_smolstr ! ("{}" , & it . text () [1 ..])) }) . collect () }) . unwrap_or_default () ; ctx . lifetime_stacks . push (params) ; } if let Some (node) = ast :: ExternBlock :: cast (node . clone ()) { ctx . extern_block_parent = Some (node) ; } Some (node) } WalkEvent :: Leave (n) => { if ast :: AnyHasGenericParams :: can_cast (n . kind ()) { ctx . lifetime_stacks . pop () ; } if ast :: ExternBlock :: can_cast (n . kind ()) { ctx . extern_block_parent = None ; } None } } }
    };
}

handle_event!();