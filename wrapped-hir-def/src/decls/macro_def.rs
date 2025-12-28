macro_rules! deps {
    () => {
        MacroId!();
        DefDatabase!();
        MacroRulesLoc!();
        Macro2Loc!();
        MacroExpander!();
    };
}

macro_rules! macro_def {
    () => {
        deps!();
        fn macro_def (db : & dyn DefDatabase , id : MacroId) -> MacroDefId { let kind = | expander , file_id , m | { let in_file = InFile :: new (file_id , m) ; match expander { MacroExpander :: Declarative => MacroDefKind :: Declarative (in_file) , MacroExpander :: BuiltIn (it) => MacroDefKind :: BuiltIn (in_file , it) , MacroExpander :: BuiltInAttr (it) => MacroDefKind :: BuiltInAttr (in_file , it) , MacroExpander :: BuiltInDerive (it) => MacroDefKind :: BuiltInDerive (in_file , it) , MacroExpander :: BuiltInEager (it) => MacroDefKind :: BuiltInEager (in_file , it) , } } ; match id { MacroId :: Macro2Id (it) => { let loc : Macro2Loc = it . lookup (db) ; MacroDefId { krate : loc . container . krate , kind : kind (loc . expander , loc . id . file_id , loc . id . value . upcast ()) , local_inner : false , allow_internal_unsafe : loc . allow_internal_unsafe , edition : loc . edition , } } MacroId :: MacroRulesId (it) => { let loc : MacroRulesLoc = it . lookup (db) ; MacroDefId { krate : loc . container . krate , kind : kind (loc . expander , loc . id . file_id , loc . id . value . upcast ()) , local_inner : loc . flags . contains (MacroRulesLocFlags :: LOCAL_INNER) , allow_internal_unsafe : loc . flags . contains (MacroRulesLocFlags :: ALLOW_INTERNAL_UNSAFE) , edition : loc . edition , } } MacroId :: ProcMacroId (it) => { let loc = it . lookup (db) ; MacroDefId { krate : loc . container . krate , kind : MacroDefKind :: ProcMacro (loc . id , loc . expander , loc . kind) , local_inner : false , allow_internal_unsafe : false , edition : loc . edition , } } } }
    };
}

macro_def!();