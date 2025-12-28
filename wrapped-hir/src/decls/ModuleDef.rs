macro_rules! deps {
    () => {
        Trait!();
        Macro!();
        Module!();
        Adt!();
        BuiltinType!();
        Static!();
        Const!();
        Variant!();
        Function!();
        TypeAlias!();
    };
}

macro_rules! ModuleDef {
    () => {
        deps!();
        # [doc = " The defs which can be visible in the module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ModuleDef { Module (Module) , Function (Function) , Adt (Adt) , Variant (Variant) , Const (Const) , Static (Static) , Trait (Trait) , TypeAlias (TypeAlias) , BuiltinType (BuiltinType) , Macro (Macro) , }
    };
}

ModuleDef!();