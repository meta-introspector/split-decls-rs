macro_rules! deps {
    () => {
        ExpectedSingleVersionLiteral!();
        ExpectedVersionLiteral!();
        UnknownVersionLiteral!();
        AcceptContext!();
        Stage!();
        MetaItemListParser!();
    };
}

macro_rules! parse_cfg_entry_version {
    () => {
        deps!();
        fn parse_cfg_entry_version < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , list : & MetaItemListParser < '_ > , meta_span : Span ,) -> Option < CfgEntry > { try_gate_cfg (sym :: version , meta_span , cx . sess () , cx . features_option ()) ; let Some (version) = list . single () else { cx . emit_err (session_diagnostics :: ExpectedSingleVersionLiteral { span : list . span }) ; return None ; } ; let Some (version_lit) = version . lit () else { cx . emit_err (session_diagnostics :: ExpectedVersionLiteral { span : version . span () }) ; return None ; } ; let Some (version_str) = version_lit . value_str () else { cx . emit_err (session_diagnostics :: ExpectedVersionLiteral { span : version_lit . span }) ; return None ; } ; let min_version = parse_version (version_str) . or_else (| | { cx . sess () . dcx () . emit_warn (session_diagnostics :: UnknownVersionLiteral { span : version_lit . span }) ; None }) ; Some (CfgEntry :: Version (min_version , list . span)) }
    };
}

parse_cfg_entry_version!()