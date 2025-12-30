// Generated macro for max_size_sum (function)
macro_rules! Depcrate_max_sizemax_size_sum {
() => {
// Module: crate::max_size
// Provides: {"max_size_sum"}
// Dependencies: {}
# [doc = " Generate a constant expression that sums up the maximum size of the type."] fn max_size_sum (data : & Data , span : Span) -> Result < TokenStream , syn :: Error > { match data { Data :: Struct (data) => Ok (sum_fields (& data . fields)) , Data :: Enum (data) => { let variant_count = data . variants . len () ; let recurse = data . variants . iter () . map (| v | sum_fields (& v . fields)) ; let discriminant_size = varint_size_discriminant (variant_count as u32) as usize ; let max = recurse . fold (quote ! (0) , | acc , x | { quote ! { { let lhs = # acc ; let rhs = # x ; if lhs > rhs { lhs } else { rhs } } } }) ; Ok (quote ! { # discriminant_size + # max }) } Data :: Union (_) => Err (syn :: Error :: new (span , "unions are not supported by `postcard::MaxSize`" ,)) , } }
};
}
