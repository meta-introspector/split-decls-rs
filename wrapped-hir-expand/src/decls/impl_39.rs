macro_rules! deps {
    () => {
        ExpandDatabase!();
        MacroDefKind!();
        ProcMacro!();
        TokenExpander!();
        MacroDefId!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl TokenExpander { fn macro_expander (db : & dyn ExpandDatabase , id : MacroDefId) -> TokenExpander { match id . kind { MacroDefKind :: Declarative (ast_id) => { TokenExpander :: DeclarativeMacro (db . decl_macro_expander (id . krate , ast_id)) } MacroDefKind :: BuiltIn (_ , expander) => TokenExpander :: BuiltIn (expander) , MacroDefKind :: BuiltInAttr (_ , expander) => TokenExpander :: BuiltInAttr (expander) , MacroDefKind :: BuiltInDerive (_ , expander) => TokenExpander :: BuiltInDerive (expander) , MacroDefKind :: BuiltInEager (_ , expander) => TokenExpander :: BuiltInEager (expander) , MacroDefKind :: ProcMacro (_ , expander , _) => TokenExpander :: ProcMacro (expander) , } } }
    };
}

impl_39!();