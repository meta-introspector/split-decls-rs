// Generated macro for write_where_clause (function)
macro_rules! Depcrate_displaywrite_where_clause {
() => {
// Module: crate::display
// Provides: {"write_where_clause"}
// Dependencies: {}
fn write_where_clause < 'db > (def : GenericDefId , f : & mut HirFormatter < '_ , 'db > ,) -> Result < bool , HirDisplayError > { let (params , store) = f . db . generic_params_and_store (def) ; if ! has_disaplayable_predicates (f . db , & params , & store) { return Ok (false) ; } f . write_str ("\nwhere") ? ; write_where_predicates (& params , & store , f) ? ; Ok (true) }
};
}
