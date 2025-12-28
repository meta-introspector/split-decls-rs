macro_rules! deps {
    () => {
        AcceptContext!();
        InvalidPredicate!();
        MetaItemParser!();
        CfgPredicateIdentifier!();
        ArgParser!();
        MetaItemOrLitParser!();
        Stage!();
    };
}

macro_rules! parse_cfg_entry {
    () => {
        deps!();
        pub (crate) fn parse_cfg_entry < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , item : & MetaItemOrLitParser < '_ > ,) -> Option < CfgEntry > { Some (match item { MetaItemOrLitParser :: MetaItemParser (meta) => match meta . args () { ArgParser :: List (list) => match meta . path () . word_sym () { Some (sym :: not) => { let Some (single) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; CfgEntry :: Not (Box :: new (parse_cfg_entry (cx , single) ?) , list . span) } Some (sym :: any) => CfgEntry :: Any (list . mixed () . flat_map (| sub_item | parse_cfg_entry (cx , sub_item)) . collect () , list . span ,) , Some (sym :: all) => CfgEntry :: All (list . mixed () . flat_map (| sub_item | parse_cfg_entry (cx , sub_item)) . collect () , list . span ,) , Some (sym :: target) => parse_cfg_entry_target (cx , list , meta . span ()) ? , Some (sym :: version) => parse_cfg_entry_version (cx , list , meta . span ()) ? , _ => { cx . emit_err (session_diagnostics :: InvalidPredicate { span : meta . span () , predicate : meta . path () . to_string () , }) ; return None ; } } , a @ (ArgParser :: NoArgs | ArgParser :: NameValue (_)) => { let Some (name) = meta . path () . word_sym () else { cx . emit_err (session_diagnostics :: CfgPredicateIdentifier { span : meta . path () . span () , }) ; return None ; } ; parse_name_value (name , meta . path () . span () , a . name_value () , meta . span () , cx) ? } } , MetaItemOrLitParser :: Lit (lit) => match lit . kind { LitKind :: Bool (b) => CfgEntry :: Bool (b , lit . span) , _ => { cx . emit_err (session_diagnostics :: CfgPredicateIdentifier { span : lit . span }) ; return None ; } } , MetaItemOrLitParser :: Err (_ , _) => return None , }) }
    };
}

parse_cfg_entry!();