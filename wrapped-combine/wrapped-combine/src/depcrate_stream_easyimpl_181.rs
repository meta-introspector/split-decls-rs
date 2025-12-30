// Generated macro for impl_181 (impl)
macro_rules! Depcrate_stream_easyimpl_181 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_181"}
// Dependencies: {}
impl < T : fmt :: Display , R : fmt :: Display > fmt :: Display for Error < T , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Error :: Unexpected (ref c) => write ! (f , "Unexpected {}" , c) , Error :: Expected (ref s) => write ! (f , "Expected {}" , s) , Error :: Message (ref msg) => msg . fmt (f) , Error :: Other (ref err) => err . fmt (f) , } } }
};
}
