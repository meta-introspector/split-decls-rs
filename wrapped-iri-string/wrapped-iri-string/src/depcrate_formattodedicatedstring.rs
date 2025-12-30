// Generated macro for ToDedicatedString (trait)
macro_rules! Depcrate_formatToDedicatedString {
() => {
// Module: crate::format
// Provides: {"ToDedicatedString"}
// Dependencies: {}
# [doc = " A trait for types that can be converted to a dedicated allocated string types."] # [cfg (feature = "alloc")] pub trait ToDedicatedString { # [doc = " Conversion target type."] type Target ; # [doc = " Converts the value to the allocated string."] fn try_to_dedicated_string (& self) -> Result < Self :: Target , TryReserveError > ; # [doc = " Converts the value to the allocated string."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if memory allocation error occured."] # [inline] # [must_use] fn to_dedicated_string (& self) -> Self :: Target { self . try_to_dedicated_string () . expect ("failed to allocate enough memory") } }
};
}
