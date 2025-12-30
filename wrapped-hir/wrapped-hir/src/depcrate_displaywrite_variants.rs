// Generated macro for write_variants (function)
macro_rules! Depcrate_displaywrite_variants {
() => {
// Module: crate::display
// Provides: {"write_variants"}
// Dependencies: {}
fn write_variants < 'db > (variants : & [Variant] , has_where_clause : bool , limit : usize , f : & mut HirFormatter < '_ , 'db > ,) -> Result < () , HirDisplayError > { let count = variants . len () . min (limit) ; f . write_char (if ! has_where_clause { ' ' } else { '\n' }) ? ; if count == 0 { let variants = if variants . is_empty () { "{}" } else { "{ /* … */ }" } ; f . write_str (variants) ? ; } else { f . write_str ("{\n") ? ; for variant in & variants [.. count] { write ! (f , "    {}" , variant . name (f . db) . display (f . db , f . edition ())) ? ; match variant . kind (f . db) { StructKind :: Tuple => { let fields_str = if variant . fields (f . db) . is_empty () { "()" } else { "( /* … */ )" } ; f . write_str (fields_str) ? ; } StructKind :: Record => { let fields_str = if variant . fields (f . db) . is_empty () { " {}" } else { " { /* … */ }" } ; f . write_str (fields_str) ? ; } StructKind :: Unit => { } } f . write_str (",\n") ? ; } if variants . len () > count { f . write_str ("    /* … */\n") ? ; } f . write_str ("}") ? ; } Ok (()) }
};
}
