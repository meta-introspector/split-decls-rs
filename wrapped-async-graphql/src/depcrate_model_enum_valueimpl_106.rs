// Generated macro for impl_106 (impl)
macro_rules! Depcrate_model_enum_valueimpl_106 {
() => {
// Module: crate::model::enum_value
// Provides: {"impl_106"}
// Dependencies: {}
# [doc = " One possible value for a given Enum. Enum values are unique values, not a"] # [doc = " placeholder for a string or numeric value. However an Enum value is returned"] # [doc = " in a JSON response as a string."] # [Object (internal , name = "__EnumValue")] impl __EnumValue < '_ > { # [inline] async fn name (& self) -> & str { & self . value . name } # [inline] async fn description (& self) -> Option < & str > { self . value . description . as_deref () } # [inline] async fn is_deprecated (& self) -> bool { self . value . deprecation . is_deprecated () } # [inline] async fn deprecation_reason (& self) -> Option < & str > { self . value . deprecation . reason () } }
};
}
