macro_rules! deps {
    () => {
        Macro2!();
        Union!();
        TypeAlias!();
        Const!();
        Struct!();
        Enum!();
        Trait!();
        ExternBlock!();
        Function!();
        MacroRules!();
        Static!();
        Impl!();
        MacroCall!();
    };
}

macro_rules! SmallModItem {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] enum SmallModItem { Const (Const) , Enum (Enum) , ExternBlock (ExternBlock) , Function (Function) , Impl (Impl) , Macro2 (Macro2) , MacroCall (MacroCall) , MacroRules (MacroRules) , Static (Static) , Struct (Struct) , Trait (Trait) , TypeAlias (TypeAlias) , Union (Union) , }
    };
}

SmallModItem!()