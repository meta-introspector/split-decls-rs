macro_rules! deps {
    () => {
        CompletionItemRefMode!();
        CompletionRelevance!();
        Builder!();
        CompletionItemKind!();
        CompletionItemLabel!();
    };
}

macro_rules! CompletionItem {
    () => {
        deps!();
        # [doc = " `CompletionItem` describes a single completion entity which expands to 1 or more entries in the"] # [doc = " editor pop-up."] # [doc = ""] # [doc = " It is basically a POD with various properties. To construct a [`CompletionItem`],"] # [doc = " use [`Builder::new`] method and the [`Builder`] struct."] # [derive (Clone , UpmapFromRaFixture)] # [non_exhaustive] pub struct CompletionItem { # [doc = " Label in the completion pop up which identifies completion."] pub label : CompletionItemLabel , # [doc = " Range of identifier that is being completed."] # [doc = ""] # [doc = " It should be used primarily for UI, but we also use this to convert"] # [doc = " generic TextEdit into LSP's completion edit (see conv.rs)."] # [doc = ""] # [doc = " `source_range` must contain the completion offset. `text_edit` should"] # [doc = " start with what `source_range` points to, or VSCode will filter out the"] # [doc = " completion silently."] pub source_range : TextRange , # [doc = " What happens when user selects this item."] # [doc = ""] # [doc = " Typically, replaces `source_range` with new identifier."] pub text_edit : TextEdit , pub is_snippet : bool , # [doc = " What item (struct, function, etc) are we completing."] pub kind : CompletionItemKind , # [doc = " Lookup is used to check if completion item indeed can complete current"] # [doc = " ident."] # [doc = ""] # [doc = " That is, in `foo.bar$0` lookup of `abracadabra` will be accepted (it"] # [doc = " contains `bar` sub sequence), and `quux` will rejected."] pub lookup : SmolStr , # [doc = " Additional info to show in the UI pop up."] pub detail : Option < String > , pub documentation : Option < Documentation > , # [doc = " Whether this item is marked as deprecated"] pub deprecated : bool , # [doc = " If completing a function call, ask the editor to show parameter popup"] # [doc = " after completion."] pub trigger_call_info : bool , # [doc = " We use this to sort completion. Relevance records facts like \"do the"] # [doc = " types align precisely?\". We can't sort by relevances directly, they are"] # [doc = " only partially ordered."] # [doc = ""] # [doc = " Note that Relevance ignores fuzzy match score. We compute Relevance for"] # [doc = " all possible items, and then separately build an ordered completion list"] # [doc = " based on relevance and fuzzy matching with the already typed identifier."] pub relevance : CompletionRelevance , # [doc = " Indicates that a reference or mutable reference to this variable is a"] # [doc = " possible match."] pub ref_match : Option < (CompletionItemRefMode , TextSize) > , # [doc = " The import data to add to completion's edits."] pub import_to_add : SmallVec < [String ; 1] > , }
    };
}

CompletionItem!()