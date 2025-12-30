// Generated macro for ToSqlx (trait)
macro_rules! Depcrate_wrappersToSqlx {
() => {
// Module: crate::wrappers
// Provides: {"ToSqlx"}
// Dependencies: {}
# [doc = " A trait for convenient conversions from Jiff types to SQLx types."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows how to convert a [`jiff::Timestamp`] to a [`Timestamp`]:"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff_sqlx::ToSqlx;"] # [doc = ""] # [doc = " let ts: jiff::Timestamp = \"2025-02-20T17:00-05\".parse()?;"] # [doc = " let wrapper = ts.to_sqlx();"] # [doc = " assert_eq!(format!(\"{wrapper:?}\"), \"Timestamp(2025-02-20T22:00:00Z)\");"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub trait ToSqlx { # [doc = " The wrapper type to convert to."] type Target ; # [doc = " A conversion method that converts a Jiff type to a SQLx wrapper type."] fn to_sqlx (self) -> Self :: Target ; }
};
}
