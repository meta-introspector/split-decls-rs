// Generated macro for is_ignorable_export (function)
macro_rules! Depcrate_redundant_pub_crateis_ignorable_export {
() => {
// Module: crate::redundant_pub_crate
// Provides: {"is_ignorable_export"}
// Dependencies: {}
fn is_ignorable_export < 'tcx > (item : & 'tcx Item < 'tcx >) -> bool { if let ItemKind :: Use (path , kind) = item . kind { let ignore = matches ! (path . res . macro_ns , Some (Res :: Def (DefKind :: Macro (_) , _))) || kind == UseKind :: ListStem ; if ignore { return true ; } } else if let ItemKind :: Macro (..) = item . kind { return true ; } false }
};
}
