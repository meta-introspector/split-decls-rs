macro_rules! deps {
    () => {
        CompletionRelevance!();
        CompletionItemKind!();
        CompletionItemRefMode!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " A helper to make `CompletionItem`s."] # [must_use] # [derive (Clone)] pub (crate) struct Builder { source_range : TextRange , imports_to_add : SmallVec < [LocatedImport ; 1] > , trait_name : Option < SmolStr > , doc_aliases : Vec < SmolStr > , label : SmolStr , insert_text : Option < String > , is_snippet : bool , detail : Option < String > , documentation : Option < Documentation > , lookup : Option < SmolStr > , kind : CompletionItemKind , text_edit : Option < TextEdit > , deprecated : bool , trigger_call_info : bool , relevance : CompletionRelevance , ref_match : Option < (CompletionItemRefMode , TextSize) > , edition : Edition , }
    };
}

Builder!()