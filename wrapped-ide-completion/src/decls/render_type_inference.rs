macro_rules! deps {
    () => {
        CompletionRelevance!();
        CompletionItemKind!();
        CompletionRelevanceTypeMatch!();
        CompletionContext!();
        CompletionItem!();
    };
}

macro_rules! render_type_inference {
    () => {
        deps!();
        pub (crate) fn render_type_inference (ty_string : String , ctx : & CompletionContext < '_ > ,) -> CompletionItem { let mut builder = CompletionItem :: new (CompletionItemKind :: InferredType , ctx . source_range () , ty_string , ctx . edition ,) ; builder . set_relevance (CompletionRelevance { type_match : Some (CompletionRelevanceTypeMatch :: Exact) , exact_name_match : true , .. Default :: default () }) ; builder . build (ctx . db) }
    };
}

render_type_inference!();