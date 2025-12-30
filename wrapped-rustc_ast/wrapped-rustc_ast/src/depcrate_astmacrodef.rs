// Generated macro for MacroDef (struct)
macro_rules! Depcrate_astMacroDef {
() => {
// Module: crate::ast
// Provides: {"MacroDef"}
// Dependencies: {}
# [doc = " Represents a macro definition."] # [derive (Clone , Encodable , Decodable , Debug , HashStable_Generic , Walkable)] pub struct MacroDef { pub body : Box < DelimArgs > , # [doc = " `true` if macro was defined with `macro_rules`."] pub macro_rules : bool , }
};
}
