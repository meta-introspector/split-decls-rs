// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: ExtraColor { style , word } => { write ! (fmt , "Error parsing style \"{style}\": extra color \"{word}\"") } Self :: UnknownWord { style , word } => { write ! (fmt , "Error parsing style \"{style}\": unknown word: \"{word}\"") } } } }
};
}
