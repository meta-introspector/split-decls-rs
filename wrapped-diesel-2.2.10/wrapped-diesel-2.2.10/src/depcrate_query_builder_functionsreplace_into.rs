// Generated macro for replace_into (function)
macro_rules! Depcrate_query_builder_functionsreplace_into {
() => {
// Module: crate::query_builder::functions
// Provides: {"replace_into"}
// Dependencies: {}
# [doc = " Creates a `REPLACE` statement."] # [doc = ""] # [doc = " If a constraint violation fails, the database will attempt to replace the"] # [doc = " offending row instead. This function is only available with MySQL and"] # [doc = " SQLite."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # include!(\"../doctest_setup.rs\");"] # [doc = " #"] # [doc = " # #[cfg(not(feature = \"postgres\"))]"] # [doc = " # fn main() {"] # [doc = " #     use schema::users::dsl::*;"] # [doc = " #     use diesel::{insert_into, replace_into};"] # [doc = " #"] # [doc = " #     let conn = &mut establish_connection();"] # [doc = " #     diesel::sql_query(\"DELETE FROM users\").execute(conn).unwrap();"] # [doc = " replace_into(users)"] # [doc = "     .values(&vec!["] # [doc = "         (id.eq(1), name.eq(\"Sean\")),"] # [doc = "         (id.eq(2), name.eq(\"Tess\")),"] # [doc = "     ])"] # [doc = "     .execute(conn)"] # [doc = "     .unwrap();"] # [doc = ""] # [doc = " replace_into(users)"] # [doc = "     .values((id.eq(1), name.eq(\"Jim\")))"] # [doc = "     .execute(conn)"] # [doc = "     .unwrap();"] # [doc = ""] # [doc = " let names = users.select(name).order(id).load::<String>(conn);"] # [doc = " assert_eq!(Ok(vec![\"Jim\".into(), \"Tess\".into()]), names);"] # [doc = " # }"] # [doc = " # #[cfg(feature = \"postgres\")] fn main() {}"] pub fn replace_into < T : Table > (target : T) -> IncompleteReplaceStatement < T > { IncompleteInsertStatement :: new (target , Replace) }
};
}
