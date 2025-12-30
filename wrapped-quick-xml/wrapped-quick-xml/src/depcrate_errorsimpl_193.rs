// Generated macro for impl_193 (impl)
macro_rules! Depcrate_errorsimpl_193 {
() => {
// Module: crate::errors
// Provides: {"impl_193"}
// Dependencies: {}
impl fmt :: Display for IllFormedError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: MissingDeclVersion (None) => { f . write_str ("an XML declaration does not contain `version` attribute") } Self :: MissingDeclVersion (Some (attr)) => { write ! (f , "an XML declaration must start with `version` attribute, but in starts with `{}`" , attr) } Self :: MissingDoctypeName => { f . write_str ("`<!DOCTYPE>` declaration does not contain a name of a document type") } Self :: MissingEndTag (tag) => write ! (f , "start tag not closed: `</{}>` not found before end of input" , tag ,) , Self :: UnmatchedEndTag (tag) => { write ! (f , "close tag `</{}>` does not match any open tag" , tag) } Self :: MismatchedEndTag { expected , found } => write ! (f , "expected `</{}>`, but `</{}>` was found" , expected , found ,) , Self :: DoubleHyphenInComment => { f . write_str ("forbidden string `--` was found in a comment") } Self :: UnclosedReference => f . write_str ("entity or character reference not closed: `;` not found before end of input" ,) , } } }
};
}
