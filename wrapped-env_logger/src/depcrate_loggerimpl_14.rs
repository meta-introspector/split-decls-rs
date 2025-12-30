// Generated macro for impl_14 (impl)
macro_rules! Depcrate_loggerimpl_14 {
() => {
// Module: crate::logger
// Provides: {"impl_14"}
// Dependencies: {}
impl Log for Logger { fn enabled (& self , metadata : & Metadata < '_ >) -> bool { self . filter . enabled (metadata) } fn log (& self , record : & Record < '_ >) { if self . matches (record) { thread_local ! { static FORMATTER : RefCell < Option < Formatter >> = const { RefCell :: new (None) } ; } let print = | formatter : & mut Formatter , record : & Record < '_ > | { let _ = self . format . format (formatter , record) . and_then (| _ | formatter . print (& self . writer)) ; formatter . clear () ; } ; let printed = FORMATTER . try_with (| tl_buf | { if let Ok (mut tl_buf) = tl_buf . try_borrow_mut () { if let Some (ref mut formatter) = * tl_buf { if formatter . write_style () != self . writer . write_style () { * formatter = Formatter :: new (& self . writer) ; } print (formatter , record) ; } else { let mut formatter = Formatter :: new (& self . writer) ; print (& mut formatter , record) ; * tl_buf = Some (formatter) ; } } else { print (& mut Formatter :: new (& self . writer) , record) ; } }) . is_ok () ; if ! printed { print (& mut Formatter :: new (& self . writer) , record) ; } } } fn flush (& self) { } }
};
}
