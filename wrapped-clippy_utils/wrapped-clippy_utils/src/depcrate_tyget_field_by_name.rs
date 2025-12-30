// Generated macro for get_field_by_name (function)
macro_rules! Depcrate_tyget_field_by_name {
() => {
// Module: crate::ty
// Provides: {"get_field_by_name"}
// Dependencies: {}
# [doc = " Gets the type of a field by name."] pub fn get_field_by_name < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , name : Symbol) -> Option < Ty < 'tcx > > { match * ty . kind () { ty :: Adt (def , args) if def . is_union () || def . is_struct () => def . non_enum_variant () . fields . iter () . find (| f | f . name == name) . map (| f | f . ty (tcx , args)) , ty :: Tuple (args) => name . as_str () . parse :: < usize > () . ok () . and_then (| i | args . get (i) . copied ()) , _ => None , } }
};
}
