// Generated macro for impl_277 (impl)
macro_rules! Depcrate_tests_itemimpl_277 {
() => {
// Module: crate::tests::item
// Provides: {"impl_277"}
// Dependencies: {}
impl CompletionItem { pub (crate) fn new (kind : impl Into < CompletionItemKind > , source_range : TextRange , label : impl Into < SmolStr > , edition : Edition ,) -> Builder { let label = label . into () ; Builder { source_range , label , insert_text : None , is_snippet : false , trait_name : None , detail : None , documentation : None , lookup : None , kind : kind . into () , text_edit : None , deprecated : false , trigger_call_info : false , relevance : CompletionRelevance :: default () , ref_match : None , imports_to_add : Default :: default () , doc_aliases : vec ! [] , edition , } } # [doc = " What string is used for filtering."] pub fn lookup (& self) -> & str { self . lookup . as_str () } pub fn ref_match (& self) -> Option < (String , ide_db :: text_edit :: Indel , CompletionRelevance) > { let mut relevance = self . relevance ; relevance . type_match = Some (CompletionRelevanceTypeMatch :: Exact) ; self . ref_match . map (| (mode , offset) | { let prefix = match mode { CompletionItemRefMode :: Reference (Mutability :: Shared) => "&" , CompletionItemRefMode :: Reference (Mutability :: Mut) => "&mut " , CompletionItemRefMode :: Dereference => "*" , } ; let label = format ! ("{prefix}{}" , self . label . primary) ; (label , ide_db :: text_edit :: Indel :: insert (offset , String :: from (prefix)) , relevance) }) } }
};
}
