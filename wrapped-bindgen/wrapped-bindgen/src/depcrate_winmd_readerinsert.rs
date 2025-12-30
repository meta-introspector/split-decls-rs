// Generated macro for insert (function)
macro_rules! Depcrate_winmd_readerinsert {
() => {
// Module: crate::winmd::reader
// Provides: {"insert"}
// Dependencies: {}
fn insert (types : & mut HashMap < & 'static str , Vec < Type > > , name : & 'static str , ty : Type) { types . entry (name) . or_default () . push (ty) ; }
};
}
