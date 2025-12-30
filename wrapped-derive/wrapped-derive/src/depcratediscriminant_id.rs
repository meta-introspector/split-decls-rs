// Generated macro for discriminant_id (function)
macro_rules! Depcratediscriminant_id {
() => {
// Module: crate
// Provides: {"discriminant_id"}
// Dependencies: {}
# [doc = " Returns identifier to define a constant equal to the discriminant of a variant"] fn discriminant_id (variant : & Ident) -> Ident { Ident :: new (& format ! ("__TLS_CODEC_{variant}") , Span :: call_site ()) }
};
}
