// Generated macro for display_fmt_path (function)
macro_rules! Depcrate_mod_pathdisplay_fmt_path {
() => {
// Module: crate::mod_path
// Provides: {"display_fmt_path"}
// Dependencies: {}
fn display_fmt_path (db : & dyn ExpandDatabase , path : & ModPath , f : & mut fmt :: Formatter < '_ > , edition : Option < Edition > ,) -> fmt :: Result { let mut first_segment = true ; let mut add_segment = | s | -> fmt :: Result { if ! first_segment { f . write_str ("::") ? ; } first_segment = false ; f . write_str (s) ? ; Ok (()) } ; match path . kind { PathKind :: Plain => { } PathKind :: SELF => add_segment ("self") ? , PathKind :: Super (n) => { for _ in 0 .. n { add_segment ("super") ? ; } } PathKind :: Crate => add_segment ("crate") ? , PathKind :: Abs => add_segment ("") ? , PathKind :: DollarCrate (_) => add_segment ("$crate") ? , } for segment in & path . segments { if ! first_segment { f . write_str ("::") ? ; } first_segment = false ; match edition { Some (edition) => segment . display (db , edition) . fmt (f) ? , None => fmt :: Display :: fmt (segment . as_str () , f) ? , } ; } Ok (()) }
};
}
