macro_rules! AssistConfig {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub struct AssistConfig { pub snippet_cap : Option < SnippetCap > , pub allowed : Option < Vec < AssistKind > > , pub insert_use : InsertUseConfig , pub prefer_no_std : bool , pub prefer_prelude : bool , pub prefer_absolute : bool , pub assist_emit_must_use : bool , pub term_search_fuel : u64 , pub term_search_borrowck : bool , pub code_action_grouping : bool , pub expr_fill_default : ExprFillDefaultMode , pub prefer_self_ty : bool , }
    };
}

AssistConfig!();