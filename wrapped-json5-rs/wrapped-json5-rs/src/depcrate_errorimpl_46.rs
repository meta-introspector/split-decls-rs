// Generated macro for impl_46 (impl)
macro_rules! Depcrate_errorimpl_46 {
() => {
// Module: crate::error
// Provides: {"impl_46"}
// Dependencies: {}
impl From < pest :: error :: Error < Rule > > for Error { fn from (err : pest :: error :: Error < Rule >) -> Self { let (line , column) = match err . line_col { pest :: error :: LineColLocation :: Pos ((l , c)) => (l , c) , pest :: error :: LineColLocation :: Span ((l , c) , (_ , _)) => (l , c) , } ; Error :: Message { msg : err . to_string () , location : Some (Location { line , column }) , } } }
};
}
