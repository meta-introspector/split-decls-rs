// Generated macro for parse_attribute_args_just_once (function)
macro_rules! Depcrate_parse_fixtureparse_attribute_args_just_once {
() => {
// Module: crate::parse::fixture
// Provides: {"parse_attribute_args_just_once"}
// Dependencies: {}
fn parse_attribute_args_just_once < 'a , T : Parse > (attributes : impl Iterator < Item = & 'a syn :: Attribute > , name : & str ,) -> (Option < T > , Vec < syn :: Error >) { let mut errors = Vec :: new () ; let val = attributes . filter (| & a | attr_is (a , name)) . map (| a | (a , a . parse_args :: < T > ())) . fold (None , | first , (a , res) | match (first , res) { (None , Ok (parsed)) => Some (parsed) , (first , Err (err)) => { errors . push (err) ; first } (first , _) => { errors . push (syn :: Error :: new_spanned (a , crate :: error :: messages :: use_more_than_once (name) ,)) ; first } }) ; (val , errors) }
};
}
