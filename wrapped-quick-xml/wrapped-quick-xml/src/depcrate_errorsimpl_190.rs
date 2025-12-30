// Generated macro for impl_190 (impl)
macro_rules! Depcrate_errorsimpl_190 {
() => {
// Module: crate::errors
// Provides: {"impl_190"}
// Dependencies: {}
impl fmt :: Display for SyntaxError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: InvalidBangMarkup => f . write_str ("unknown or missed symbol in markup") , Self :: UnclosedPIOrXmlDecl => { f . write_str ("processing instruction or xml declaration not closed: `?>` not found before end of input") } Self :: UnclosedComment => { f . write_str ("comment not closed: `-->` not found before end of input") } Self :: UnclosedDoctype => { f . write_str ("DOCTYPE not closed: `>` not found before end of input") } Self :: UnclosedCData => { f . write_str ("CDATA not closed: `]]>` not found before end of input") } Self :: UnclosedTag => f . write_str ("tag not closed: `>` not found before end of input") , } } }
};
}
