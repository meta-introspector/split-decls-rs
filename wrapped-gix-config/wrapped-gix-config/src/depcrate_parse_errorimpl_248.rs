// Generated macro for impl_248 (impl)
macro_rules! Depcrate_parse_errorimpl_248 {
() => {
// Module: crate::parse::error
// Provides: {"impl_248"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Got an unexpected token on line {} while trying to parse a {}: " , self . line_number + 1 , self . last_attempted_parser ,) ? ; let data_size = self . parsed_until . len () ; let data = std :: str :: from_utf8 (& self . parsed_until) ; match (data , data_size) { (Ok (data) , _) if data_size > 10 => { write ! (f , "'{}' ... ({} characters omitted)" , & data . chars () . take (10) . collect ::< String > () , data_size - 10) } (Ok (data) , _) => write ! (f , "'{data}'") , (Err (_) , _) => self . parsed_until . fmt (f) , } } }
};
}
