macro_rules! deps {
    () => {
        CompletionConfig!();
        CompletionFieldsToResolve!();
        CallableSnippets!();
    };
}

macro_rules! TEST_CONFIG {
    () => {
        deps!();
        pub (crate) const TEST_CONFIG : CompletionConfig < '_ > = CompletionConfig { enable_postfix_completions : true , enable_imports_on_the_fly : true , enable_self_on_the_fly : true , enable_private_editable : false , enable_term_search : true , term_search_fuel : 200 , full_function_signatures : false , callable : Some (CallableSnippets :: FillArguments) , add_semicolon_to_unit : true , snippet_cap : SnippetCap :: new (true) , insert_use : InsertUseConfig { granularity : ImportGranularity :: Crate , prefix_kind : PrefixKind :: Plain , enforce_granularity : true , group : true , skip_glob_imports : true , } , prefer_no_std : false , prefer_prelude : true , prefer_absolute : false , snippets : Vec :: new () , limit : None , fields_to_resolve : CompletionFieldsToResolve :: empty () , exclude_flyimport : vec ! [] , exclude_traits : & [] , enable_auto_await : true , enable_auto_iter : true , minicore : MiniCore :: default () , } ;
    };
}

TEST_CONFIG!();