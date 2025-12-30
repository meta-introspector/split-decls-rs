// Generated macro for parse_class_generics (function)
macro_rules! Depcrate_stmtparse_class_generics {
() => {
// Module: crate::stmt
// Provides: {"parse_class_generics"}
// Dependencies: {}
pub (crate) fn parse_class_generics (entity : & Entity < '_ > , context : & Context < '_ > ,) -> Vec < GenericWithBound > { let mut generics = Vec :: new () ; # [allow (clippy :: single_match)] immediate_children (entity , | entity , _span | match entity . get_kind () { EntityKind :: TemplateTypeParameter => { let name = entity . get_name () . expect ("template name") ; let mut bound = None ; immediate_children (& entity , | entity , _span | match entity . get_kind () { EntityKind :: ObjCClassRef | EntityKind :: TypeRef => { let ty = entity . get_type () . expect ("template type") ; let ty = PointeeTy :: parse_generic_bound (ty , context) ; bound = Some (ty) ; } EntityKind :: ObjCProtocolRef => { let definition = entity . get_definition () . expect ("template definition") ; if let Some (ty) = & mut bound { ty . add_protocol (definition , context) ; } else { error ! (? entity , "should have handled protocol type parameter") ; } } _ => error ! (? entity , "unknown type parameter child") , }) ; if let Some (ty) = & bound { if ty . is_plain_anyobject () { bound = None ; } } generics . push ((name , bound)) ; } _ => { } }) ; generics }
};
}
