macro_rules! deps {
    () => {
        SymbolName!();
    };
}

macro_rules! get_relevant_span {
    () => {
        deps!();
        # [doc = " We want to ensure that we use spans for both decls that include where the"] # [doc = " name was defined, whether that was from the link_name attribute or not."] fn get_relevant_span (tcx : TyCtxt < '_ > , fi : hir :: OwnerId) -> Span { match name_of_extern_decl (tcx , fi) { SymbolName :: Normal (_) => tcx . def_span (fi) , SymbolName :: Link (_ , annot_span) => annot_span , } }
    };
}

get_relevant_span!();