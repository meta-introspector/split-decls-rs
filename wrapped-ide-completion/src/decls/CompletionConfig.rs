macro_rules! deps {
    () => {
        CallableSnippets!();
        CompletionFieldsToResolve!();
        AutoImportExclusionType!();
        Snippet!();
    };
}

macro_rules! CompletionConfig {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct CompletionConfig < 'a > { pub enable_postfix_completions : bool , pub enable_imports_on_the_fly : bool , pub enable_self_on_the_fly : bool , pub enable_auto_iter : bool , pub enable_auto_await : bool , pub enable_private_editable : bool , pub enable_term_search : bool , pub term_search_fuel : u64 , pub full_function_signatures : bool , pub callable : Option < CallableSnippets > , pub add_semicolon_to_unit : bool , pub snippet_cap : Option < SnippetCap > , pub insert_use : InsertUseConfig , pub prefer_no_std : bool , pub prefer_prelude : bool , pub prefer_absolute : bool , pub snippets : Vec < Snippet > , pub limit : Option < usize > , pub fields_to_resolve : CompletionFieldsToResolve , pub exclude_flyimport : Vec < (String , AutoImportExclusionType) > , pub exclude_traits : & 'a [String] , pub minicore : MiniCore < 'a > , }
    };
}

CompletionConfig!();