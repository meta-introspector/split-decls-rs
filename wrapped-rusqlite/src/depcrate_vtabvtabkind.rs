// Generated macro for VTabKind (enum)
macro_rules! Depcrate_vtabVTabKind {
() => {
// Module: crate::vtab
// Provides: {"VTabKind"}
// Dependencies: {}
# [doc = " Virtual table kind"] pub enum VTabKind { # [doc = " Non-eponymous"] Default , # [doc = " [`create`](CreateVTab::create) == [`connect`](VTab::connect)"] # [doc = ""] # [doc = " See [SQLite doc](https://sqlite.org/vtab.html#eponymous_virtual_tables)"] Eponymous , # [doc = " No [`create`](CreateVTab::create) / [`destroy`](CreateVTab::destroy) or"] # [doc = " not used"] # [doc = ""] # [doc = " SQLite >= 3.9.0"] # [doc = ""] # [doc = " See [SQLite doc](https://sqlite.org/vtab.html#eponymous_only_virtual_tables)"] EponymousOnly , }
};
}
