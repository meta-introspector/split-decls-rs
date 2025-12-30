// Generated macro for ParseAttribute (trait)
macro_rules! Depcrate_optionsParseAttribute {
() => {
// Module: crate::options
// Provides: {"ParseAttribute"}
// Dependencies: {}
# [doc = " Middleware for extracting attribute values. Implementers are expected to override"] # [doc = " `parse_nested` so they can apply individual items to themselves, while `parse_attributes`"] # [doc = " is responsible for looping through distinct outer attributes and collecting errors."] pub trait ParseAttribute : Sized { fn parse_attributes (mut self , attrs : & [syn :: Attribute]) -> Result < Self > { let mut errors = Error :: accumulator () ; for attr in attrs { if attr . meta . path () == & parse_quote ! (darling) { errors . handle (parse_attr (attr , & mut self)) ; } } errors . finish_with (self) } # [doc = " Read a meta-item, and apply its values to the current instance."] fn parse_nested (& mut self , mi : & syn :: Meta) -> Result < () > ; }
};
}
