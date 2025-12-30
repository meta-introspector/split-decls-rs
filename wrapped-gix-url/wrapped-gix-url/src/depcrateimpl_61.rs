// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
# [doc = " Instantiation"] impl Url { # [doc = " Create a new instance from the given parts, including a password, which will be validated by parsing them back."] pub fn from_parts (scheme : Scheme , user : Option < String > , password : Option < String > , host : Option < String > , port : Option < u16 > , path : BString , serialize_alternative_form : bool ,) -> Result < Self , parse :: Error > { parse (Url { scheme , user , password , host , port , path , serialize_alternative_form , } . to_bstring () . as_ref () ,) } }
};
}
