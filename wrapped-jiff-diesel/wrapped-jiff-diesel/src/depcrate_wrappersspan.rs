// Generated macro for Span (struct)
macro_rules! Depcrate_wrappersSpan {
() => {
// Module: crate::wrappers
// Provides: {"Span"}
// Dependencies: {}
# [doc = " A wrapper type for [`jiff::Span`]."] # [doc = ""] # [doc = " # PostgreSQL: Limited support"] # [doc = ""] # [doc = " This type _only_ has a [`diesel::deserialize::FromSql`] trait implementation"] # [doc = " for PostgreSQL. The reason for this is that encoding an arbitrary"] # [doc = " `Span` into a PostgreSQL interval requires a relative datetime."] # [doc = " Therefore, users wanting to store a `Span` will need to explicitly use a"] # [doc = " [`diesel::pg::data_types::PgInterval`] at least at encoding time."] # [derive (Clone , Copy , Debug , diesel :: deserialize :: FromSqlRow)] pub struct Span (jiff :: Span) ;
};
}
