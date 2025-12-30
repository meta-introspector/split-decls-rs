// Generated macro for GenericParamKind (enum)
macro_rules! Depcrate_astGenericParamKind {
() => {
// Module: crate::ast
// Provides: {"GenericParamKind"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum GenericParamKind { # [doc = " A lifetime definition (e.g., `'a: 'b + 'c + 'd`)."] Lifetime , Type { default : Option < Box < Ty > > , } , Const { ty : Box < Ty > , # [doc = " Span of the whole parameter definition, including default."] span : Span , # [doc = " Optional default value for the const generic param."] default : Option < AnonConst > , } , }
};
}
