// Generated macro for get (function)
macro_rules! Depcrate_internals_cratenameget {
() => {
// Module: crate::internals::cratename
// Provides: {"get"}
// Dependencies: {}
pub (crate) fn get (attrs : & [Attribute]) -> Result < Path , Error > { let path = item :: get_crate (attrs) ? ; match path { Some (path) => Ok (path) , None => { let ident = get_from_cargo () ; Ok (ident . into ()) } } }
};
}
