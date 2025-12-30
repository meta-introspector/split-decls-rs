// Generated macro for sql (module)
macro_rules! Depcratesql {
() => {
// Module: crate
// Provides: {"sql"}
// Dependencies: {}
# [doc = " Grammar rules of an SQL parser"] # [allow (missing_docs)] pub mod sql { # [doc = " SQL parser."] # [doc = " Grammar is a tinkered version of the one used in distributed SQL executor module named"] # [doc = " [sbroad](https://git.picodata.io/picodata/picodata/sbroad/-/blob/main/sbroad-core/src/frontend/sql/query.pest)."] # [doc = " Being a submodule of [Picodata](https://git.picodata.io/picodata/picodata/picodata) (that"] # [doc = " operates with Tarantool database) it tries to simulate SQLite flavour (Tarantool uses"] # [doc = " SQLite to execute SQL queries)."] # [derive (Parser)] # [grammar = "grammars/sql.pest"] pub struct SqlParser ; }
};
}
