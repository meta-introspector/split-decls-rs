// Generated macro for parse_attr (function)
macro_rules! Depcrate_optionsparse_attr {
() => {
// Module: crate::options
// Provides: {"parse_attr"}
// Dependencies: {}
fn parse_attr < T : ParseAttribute > (attr : & syn :: Attribute , target : & mut T) -> Result < () > { let mut errors = Error :: accumulator () ; match & attr . meta { syn :: Meta :: List (data) => { for item in NestedMeta :: parse_meta_list (data . tokens . clone ()) ? { if let NestedMeta :: Meta (ref mi) = item { errors . handle (target . parse_nested (mi)) ; } else { panic ! ("Wasn't able to parse: `{:?}`" , item) ; } } errors . finish () } item => panic ! ("Wasn't able to parse: `{:?}`" , item) , } }
};
}
