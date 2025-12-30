// Generated macro for CopyFormat (enum)
macro_rules! Depcrate_pg_query_builder_copyCopyFormat {
() => {
// Module: crate::pg::query_builder::copy
// Provides: {"CopyFormat"}
// Dependencies: {}
# [doc = " Describes the format used by `COPY FROM` or `COPY TO`"] # [doc = " statements"] # [doc = ""] # [doc = " See [the postgresql documentation](https://www.postgresql.org/docs/current/sql-copy.html)"] # [doc = " for details about the different formats"] # [derive (Default , Debug , Copy , Clone)] pub enum CopyFormat { # [doc = " The postgresql text format"] # [doc = ""] # [doc = " This format is the default if no format is explicitly set"] # [default] Text , # [doc = " Represents the data as comma separated values (CSV)"] Csv , # [doc = " The postgresql binary format"] Binary , }
};
}
