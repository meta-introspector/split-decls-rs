macro_rules! deps {
    () => {
        TypeAlias!();
        Static!();
        Variant!();
        Trait!();
        BuiltinType!();
        Const!();
        Adt!();
        Function!();
        Macro!();
        Module!();
    };
}

macro_rules! ModuleDef {
    () => {
        deps!();
        # [doc = " The defs which can be visible in the module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ModuleDef { Module (Module) , Function (Function) , Adt (Adt) , Variant (Variant) , Const (Const) , Static (Static) , Trait (Trait) , TypeAlias (TypeAlias) , BuiltinType (BuiltinType) , Macro (Macro) , }
    };
}

ModuleDef!()