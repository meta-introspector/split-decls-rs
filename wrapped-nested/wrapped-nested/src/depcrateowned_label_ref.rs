// Generated macro for owned_label_ref (function)
macro_rules! Depcrateowned_label_ref {
() => {
// Module: crate
// Provides: {"owned_label_ref"}
// Dependencies: {}
fn owned_label_ref (label : & sval :: Label) -> Result < sval :: Label < 'static > > { # [cfg (feature = "alloc")] { Ok (label . to_owned ()) } # [cfg (not (feature = "alloc"))] { if let Some (label) = label . as_static_str () { Ok (sval :: Label :: new (label)) } else { Err (Error :: no_alloc ("streaming value")) } } }
};
}
