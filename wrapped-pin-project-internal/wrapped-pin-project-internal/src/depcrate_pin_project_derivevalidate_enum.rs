// Generated macro for validate_enum (function)
macro_rules! Depcrate_pin_project_derivevalidate_enum {
() => {
// Module: crate::pin_project::derive
// Provides: {"validate_enum"}
// Dependencies: {}
fn validate_enum (brace_token : token :: Brace , variants : & Variants) -> Result < () > { if variants . is_empty () { return Err (Error :: new (brace_token . span . join () , "#[pin_project] attribute may not be used on enums without variants" ,)) ; } let has_field = variants . iter () . try_fold (false , | has_field , v | { if let Some ((_ , e)) = & v . discriminant { bail ! (e , "#[pin_project] attribute may not be used on enums with discriminants") ; } else if let Some (attr) = v . attrs . find (PIN) { bail ! (attr , "#[pin] attribute may only be used on fields of structs or variants") ; } else if v . fields . is_empty () { Ok (has_field) } else { Ok (true) } }) ? ; if has_field { Ok (()) } else { bail ! (variants , "#[pin_project] attribute may not be used on enums with zero fields") ; } }
};
}
