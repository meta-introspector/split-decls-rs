macro_rules! deps {
    () => {
        Qualified!();
        PathKind!();
    };
}

macro_rules! PathCompletionCtx {
    () => {
        deps!();
        # [doc = " The state of the path we are currently completing."] # [derive (Debug)] pub (crate) struct PathCompletionCtx < 'db > { # [doc = " If this is a call with () already there (or {} in case of record patterns)"] pub (crate) has_call_parens : bool , # [doc = " If this has a macro call bang !"] pub (crate) has_macro_bang : bool , # [doc = " The qualifier of the current path."] pub (crate) qualified : Qualified < 'db > , # [doc = " The parent of the path we are completing."] pub (crate) parent : Option < ast :: Path > , # [allow (dead_code)] # [doc = " The path of which we are completing the segment"] pub (crate) path : ast :: Path , # [doc = " The path of which we are completing the segment in the original file"] pub (crate) original_path : Option < ast :: Path > , pub (crate) kind : PathKind < 'db > , # [doc = " Whether the path segment has type args or not."] pub (crate) has_type_args : bool , # [doc = " Whether the qualifier comes from a use tree parent or not"] pub (crate) use_tree_parent : bool , }
    };
}

PathCompletionCtx!();