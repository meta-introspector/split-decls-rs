// Generated macro for Span (struct)
macro_rules! Depcrate_wrappersSpan {
() => {
// Module: crate::wrappers
// Provides: {"Span"}
// Dependencies: {}
# [doc = " A wrapper type for [`jiff::Span`]."] # [doc = ""] # [doc = " # PostgreSQL: Limited support"] # [doc = ""] # [doc = " This type _only_ has a [`sqlx_core::decode::Decode`] trait implementation"] # [doc = " for PostgreSQL. The reason for this is that encoding an arbitrary"] # [doc = " `Span` into a PostgreSQL interval requires a relative datetime."] # [doc = " Therefore, users wanting to store a `Span` will need to explicitly use a"] # [doc = " [`sqlx_postgres::types::PgInterval`] at least at encoding time."] # [derive (Clone , Copy , Debug)] pub struct Span (jiff :: Span) ;
};
}
