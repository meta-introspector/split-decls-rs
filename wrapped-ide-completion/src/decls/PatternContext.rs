macro_rules! deps {
    () => {
        PatternRefutability!();
        ParamContext!();
    };
}

macro_rules! PatternContext {
    () => {
        deps!();
        # [doc = " The state of the pattern we are completing."] # [derive (Debug , Clone , PartialEq , Eq)] pub (crate) struct PatternContext { pub (crate) refutability : PatternRefutability , pub (crate) param_ctx : Option < ParamContext > , pub (crate) has_type_ascription : bool , pub (crate) should_suggest_name : bool , pub (crate) after_if_expr : bool , pub (crate) parent_pat : Option < ast :: Pat > , pub (crate) ref_token : Option < SyntaxToken > , pub (crate) mut_token : Option < SyntaxToken > , # [doc = " The record pattern this name or ref is a field of"] pub (crate) record_pat : Option < ast :: RecordPat > , pub (crate) impl_or_trait : Option < Either < ast :: Impl , ast :: Trait > > , # [doc = " List of missing variants in a match expr"] pub (crate) missing_variants : Vec < hir :: Variant > , }
    };
}

PatternContext!()