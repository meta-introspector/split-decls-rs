// Generated macro for impl_510 (impl)
macro_rules! Depcrate_options_coreimpl_510 {
() => {
// Module: crate::options::core
// Provides: {"impl_510"}
// Dependencies: {}
impl ParseData for Core { fn parse_variant (& mut self , variant : & syn :: Variant) -> Result < () > { let v = InputVariant :: from_variant (variant , Some (self)) ? ; match self . data { Data :: Enum (ref mut variants) => { variants . push (v) ; Ok (()) } Data :: Struct (_) => panic ! ("Core::parse_variant should never be called for a struct") , } } fn parse_field (& mut self , field : & syn :: Field) -> Result < () > { let f = InputField :: from_field (field , Some (self)) ? ; match self . data { Data :: Struct (Fields { style : Style :: Unit , .. }) => panic ! ("Core::parse_field should not be called on unit") , Data :: Struct (Fields { ref mut fields , .. }) => { fields . push (f) ; Ok (()) } Data :: Enum (_) => panic ! ("Core::parse_field should never be called for an enum") , } } fn validate_body (& self , errors : & mut Accumulator) { if let Data :: Struct (fields) = & self . data { let flatten_targets : Vec < _ > = fields . iter () . filter_map (| field | { if field . flatten . is_present () { Some (field . flatten) } else { None } }) . collect () ; if flatten_targets . len () > 1 { for flatten in flatten_targets { errors . push (Error :: custom ("`#[darling(flatten)]` can only be applied to one field") . with_span (& flatten . span ()) ,) ; } } } } }
};
}
