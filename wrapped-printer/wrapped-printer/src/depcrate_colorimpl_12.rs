// Generated macro for impl_12 (impl)
macro_rules! Depcrate_colorimpl_12 {
() => {
// Module: crate::color
// Provides: {"impl_12"}
// Dependencies: {}
impl std :: fmt :: Display for ColorError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match * self { ColorError :: UnrecognizedOutType (ref name) => write ! (f , "unrecognized output type '{}'. Choose from: \
                 path, line, column, match, highlight." , name ,) , ColorError :: UnrecognizedSpecType (ref name) => write ! (f , "unrecognized spec type '{}'. Choose from: \
                 fg, bg, style, none." , name ,) , ColorError :: UnrecognizedColor (_ , ref msg) => write ! (f , "{}" , msg) , ColorError :: UnrecognizedStyle (ref name) => write ! (f , "unrecognized style attribute '{}'. Choose from: \
                 nobold, bold, nointense, intense, nounderline, \
                 underline, noitalic, italic." , name ,) , ColorError :: InvalidFormat (ref original) => write ! (f , "invalid color spec format: '{}'. Valid format is \
                 '(path|line|column|match|highlight):(fg|bg|style):(value)'." , original ,) , } } }
};
}
