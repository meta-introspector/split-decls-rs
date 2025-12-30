// Generated macro for ModuleDef (enum)
macro_rules! DepcrateModuleDef {
() => {
// Module: crate
// Provides: {"ModuleDef"}
// Dependencies: {}
# [doc = " The defs which can be visible in the module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ModuleDef { Module (Module) , Function (Function) , Adt (Adt) , Variant (Variant) , Const (Const) , Static (Static) , Trait (Trait) , TypeAlias (TypeAlias) , BuiltinType (BuiltinType) , Macro (Macro) , }
};
}
