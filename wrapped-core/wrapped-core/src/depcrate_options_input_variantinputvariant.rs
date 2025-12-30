// Generated macro for InputVariant (struct)
macro_rules! Depcrate_options_input_variantInputVariant {
() => {
// Module: crate::options::input_variant
// Provides: {"InputVariant"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct InputVariant { pub ident : syn :: Ident , attr_name : Option < String > , data : Fields < InputField > , skip : Option < bool > , # [doc = " Whether or not the variant should be used to create an instance for"] # [doc = " `FromMeta::from_word`."] pub word : Option < SpannedValue < bool > > , # [doc = " Whether or not unknown fields are acceptable in this"] allow_unknown_fields : Option < bool > , }
};
}
