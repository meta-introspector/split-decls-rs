// Generated macro for impl_257 (impl)
macro_rules! Depcrate_parse_rstestimpl_257 {
() => {
// Module: crate::parse::rstest
// Provides: {"impl_257"}
// Dependencies: {}
impl RsTestAttributes { const TRACE_VARIABLE_ATTR : & 'static str = "trace" ; const NOTRACE_VARIABLE_ATTR : & 'static str = "notrace" ; pub (crate) fn trace_me (& self , pat : & Pat) -> bool { if self . should_trace () { ! self . iter () . any (| m | Self :: is_notrace (pat , m)) } else { false } } fn is_notrace (pat : & Pat , m : & Attribute) -> bool { match m { Attribute :: Tagged (i , args) if i == Self :: NOTRACE_VARIABLE_ATTR => { args . iter () . any (| a | a == pat) } _ => false , } } pub (crate) fn should_trace (& self) -> bool { self . iter () . any (Self :: is_trace) } pub (crate) fn add_trace (& mut self , trace : Ident) { self . inner . attributes . push (Attribute :: Attr (trace)) ; } pub (crate) fn add_notraces (& mut self , notraces : Vec < Pat >) { if notraces . is_empty () { return ; } self . inner . attributes . push (Attribute :: Tagged (format_ident ! ("{}" , Self :: NOTRACE_VARIABLE_ATTR) , notraces ,)) ; } fn is_trace (m : & Attribute) -> bool { matches ! (m , Attribute :: Attr (i) if i == Self :: TRACE_VARIABLE_ATTR) } }
};
}
