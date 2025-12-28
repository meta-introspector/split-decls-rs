macro_rules! deps {
    () => {
        ProcMacro!();
        AstId!();
        CustomProcMacroExpander!();
        ProcMacroKind!();
    };
}

macro_rules! MacroDefKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum MacroDefKind { Declarative (AstId < ast :: Macro >) , BuiltIn (AstId < ast :: Macro > , BuiltinFnLikeExpander) , BuiltInAttr (AstId < ast :: Macro > , BuiltinAttrExpander) , BuiltInDerive (AstId < ast :: Macro > , BuiltinDeriveExpander) , BuiltInEager (AstId < ast :: Macro > , EagerExpander) , ProcMacro (AstId < ast :: Fn > , CustomProcMacroExpander , ProcMacroKind) , }
    };
}

MacroDefKind!();