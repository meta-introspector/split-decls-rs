macro_rules! deps {
    () => {
        Static!();
        MacroCall!();
        ExternBlock!();
        Impl!();
        Trait!();
        Macro2!();
        MacroRules!();
        Enum!();
        Function!();
        Union!();
        Const!();
        TypeAlias!();
        Struct!();
    };
}

macro_rules! SmallModItem {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] enum SmallModItem { Const (Const) , Enum (Enum) , ExternBlock (ExternBlock) , Function (Function) , Impl (Impl) , Macro2 (Macro2) , MacroCall (MacroCall) , MacroRules (MacroRules) , Static (Static) , Struct (Struct) , Trait (Trait) , TypeAlias (TypeAlias) , Union (Union) , }
    };
}

SmallModItem!();