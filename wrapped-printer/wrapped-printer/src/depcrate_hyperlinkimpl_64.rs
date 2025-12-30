// Generated macro for impl_64 (impl)
macro_rules! Depcrate_hyperlinkimpl_64 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_64"}
// Dependencies: {}
impl std :: fmt :: Display for Part { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self { Part :: Text (text) => write ! (f , "{}" , String :: from_utf8_lossy (text)) , Part :: Host => write ! (f , "{{host}}") , Part :: WSLPrefix => write ! (f , "{{wslprefix}}") , Part :: Path => write ! (f , "{{path}}") , Part :: Line => write ! (f , "{{line}}") , Part :: Column => write ! (f , "{{column}}") , } } }
};
}
