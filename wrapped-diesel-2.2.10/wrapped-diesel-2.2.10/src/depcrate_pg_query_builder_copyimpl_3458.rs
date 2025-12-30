// Generated macro for impl_3458 (impl)
macro_rules! Depcrate_pg_query_builder_copyimpl_3458 {
() => {
// Module: crate::pg::query_builder::copy
// Provides: {"impl_3458"}
// Dependencies: {}
impl CommonOptions { fn any_set (& self) -> bool { self . format . is_some () || self . freeze . is_some () || self . delimiter . is_some () || self . null . is_some () || self . quote . is_some () || self . escape . is_some () } fn walk_ast < 'b > (& 'b self , mut pass : crate :: query_builder :: AstPass < '_ , 'b , Pg > , comma : & mut & 'static str ,) { if let Some (format) = self . format { pass . push_sql (comma) ; * comma = ", " ; pass . push_sql ("FORMAT ") ; pass . push_sql (format . to_sql_format ()) ; } if let Some (freeze) = self . freeze { pass . push_sql (& format ! ("{comma}FREEZE {}" , freeze as u8)) ; * comma = ", " ; } if let Some (delimiter) = self . delimiter { pass . push_sql (& format ! ("{comma}DELIMITER '{delimiter}'")) ; * comma = ", " ; } if let Some (ref null) = self . null { pass . push_sql (comma) ; * comma = ", " ; pass . push_sql ("NULL '") ; pass . push_sql (null) ; pass . push_sql ("'") ; } if let Some (quote) = self . quote { pass . push_sql (& format ! ("{comma}QUOTE '{quote}'")) ; * comma = ", " ; } if let Some (escape) = self . escape { pass . push_sql (& format ! ("{comma}ESCAPE '{escape}'")) ; * comma = ", " ; } } }
};
}
