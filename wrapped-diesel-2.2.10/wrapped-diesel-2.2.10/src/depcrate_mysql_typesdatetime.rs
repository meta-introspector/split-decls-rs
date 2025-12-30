// Generated macro for Datetime (struct)
macro_rules! Depcrate_mysql_typesDatetime {
() => {
// Module: crate::mysql::types
// Provides: {"Datetime"}
// Dependencies: {}
# [doc = " Represents the MySQL datetime type."] # [doc = ""] # [doc = " ### [`ToSql`] impls"] # [doc = ""] # [doc = " - [`chrono::NaiveDateTime`] with `feature = \"chrono\"`"] # [doc = " - [`time::PrimitiveDateTime`] with `feature = \"time\"`"] # [doc = " - [`time::OffsetDateTime`] with `feature = \"time\"`"] # [doc = ""] # [doc = " ### [`FromSql`] impls"] # [doc = ""] # [doc = " - [`chrono::NaiveDateTime`] with `feature = \"chrono\"`"] # [doc = " - [`time::PrimitiveDateTime`] with `feature = \"time\"`"] # [doc = " - [`time::OffsetDateTime`] with `feature = \"time\"`"] # [doc = ""] # [doc = " [`ToSql`]: crate::serialize::ToSql"] # [doc = " [`FromSql`]: crate::deserialize::FromSql"] # [cfg_attr (feature = "chrono" , doc = " [`chrono::NaiveDateTime`]: chrono::naive::NaiveDateTime")] # [cfg_attr (not (feature = "chrono") , doc = " [`chrono::NaiveDateTime`]: https://docs.rs/chrono/0.4.19/chrono/naive/struct.NaiveDateTime.html")] # [cfg_attr (feature = "time" , doc = " [`time::PrimitiveDateTime`]: time::PrimitiveDateTime")] # [cfg_attr (not (feature = "time") , doc = " [`time::PrimitiveDateTime`]: https://docs.rs/time/0.3.9/time/struct.PrimitiveDateTime.html")] # [cfg_attr (feature = "time" , doc = " [`time::OffsetDateTime`]: time::OffsetDateTime")] # [cfg_attr (not (feature = "time") , doc = " [`time::OffsetDateTime`]: https://docs.rs/time/0.3.9/time/struct.OffsetDateTime.html")] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (mysql_type (name = "DateTime"))] # [cfg (feature = "mysql_backend")] pub struct Datetime ;
};
}
