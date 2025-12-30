// Generated macro for add_trait_bounds (function)
macro_rules! Depcrate_max_sizeadd_trait_bounds {
() => {
// Module: crate::max_size
// Provides: {"add_trait_bounds"}
// Dependencies: {}
# [doc = " Add a bound `T: MaxSize` to every type parameter T."] fn add_trait_bounds (mut generics : Generics) -> Generics { for param in & mut generics . params { if let GenericParam :: Type (ref mut type_param) = * param { type_param . bounds . push (parse_quote ! (:: postcard2 :: experimental :: max_size :: MaxSize)) ; } } generics }
};
}
