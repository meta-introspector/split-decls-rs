// Generated macro for parse_superclasses (function)
macro_rules! Depcrate_stmtparse_superclasses {
() => {
// Module: crate::stmt
// Provides: {"parse_superclasses"}
// Dependencies: {}
pub (crate) fn parse_superclasses < 'ty > (entity : & Entity < 'ty > , context : & Context < '_ > ,) -> Vec < (ItemIdentifier , Vec < String > , Entity < 'ty >) > { let mut current_entity = * entity ; let mut superclasses = vec ! [] ; loop { let mut superclass = None ; let mut applied_generics = Vec :: new () ; immediate_children (& current_entity , | entity , _span | match entity . get_kind () { EntityKind :: ObjCSuperClassRef => { superclass = Some (entity . get_reference () . expect ("ObjCSuperClassRef to reference entity") ,) ; } EntityKind :: TypeRef => { let name = entity . get_name () . expect ("typeref name") ; applied_generics . push (name) ; } _ => { } }) ; if let Some (superclass) = superclass { current_entity = superclass ; superclasses . push ((ItemIdentifier :: new (& superclass , context) , applied_generics , superclass ,)) ; } else { return superclasses ; } } }
};
}
