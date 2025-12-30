// Generated macro for generate_default (function)
macro_rules! Depcrate_utilsgenerate_default {
() => {
// Module: crate::utils
// Provides: {"generate_default"}
// Dependencies: {}
pub fn generate_default (default : & Option < args :: DefaultValue > , default_with : & Option < LitStr > ,) -> GeneratorResult < Option < TokenStream > > { match (default , default_with) { (Some (args :: DefaultValue :: Default) , _) => { Ok (Some (quote ! { :: std :: default :: Default :: default () })) } (Some (args :: DefaultValue :: Value (lit)) , _) => Ok (Some (generate_default_value (lit) ?)) , (None , Some (lit)) => Ok (Some (generate_default_with (lit) ?)) , (None , None) => Ok (None) , } }
};
}
