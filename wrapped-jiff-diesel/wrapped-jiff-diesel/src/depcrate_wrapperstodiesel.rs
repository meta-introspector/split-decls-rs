// Generated macro for ToDiesel (trait)
macro_rules! Depcrate_wrappersToDiesel {
() => {
// Module: crate::wrappers
// Provides: {"ToDiesel"}
// Dependencies: {}
# [doc = " A trait for convenient conversions from Jiff types to Diesel types."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows how to convert a [`jiff::Timestamp`] to a [`Timestamp`]:"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff_diesel::ToDiesel;"] # [doc = ""] # [doc = " let ts: jiff::Timestamp = \"2025-02-20T17:00-05\".parse()?;"] # [doc = " let wrapper = ts.to_diesel();"] # [doc = " assert_eq!(format!(\"{wrapper:?}\"), \"Timestamp(2025-02-20T22:00:00Z)\");"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub trait ToDiesel { # [doc = " The wrapper type to convert to."] type Target ; # [doc = " A conversion method that converts a Jiff type to a Diesel wrapper type."] fn to_diesel (self) -> Self :: Target ; }
};
}
