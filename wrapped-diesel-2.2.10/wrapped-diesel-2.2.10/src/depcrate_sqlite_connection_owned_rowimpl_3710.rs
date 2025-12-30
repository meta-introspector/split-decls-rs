// Generated macro for impl_3710 (impl)
macro_rules! Depcrate_sqlite_connection_owned_rowimpl_3710 {
() => {
// Module: crate::sqlite::connection::owned_row
// Provides: {"impl_3710"}
// Dependencies: {}
impl < 'a > Row < 'a , Sqlite > for OwnedSqliteRow { type Field < 'field > = OwnedSqliteField < 'field > where 'a : 'field , Self : 'field ; type InnerPartialRow = Self ; fn field_count (& self) -> usize { self . values . len () } fn get < 'field , I > (& 'field self , idx : I) -> Option < Self :: Field < 'field > > where 'a : 'field , Self : RowIndex < I > , { let idx = self . idx (idx) ? ; Some (OwnedSqliteField { row : self , col_idx : idx , }) } fn partial_row (& self , range : std :: ops :: Range < usize >) -> PartialRow < '_ , Self :: InnerPartialRow > { PartialRow :: new (self , range) } }
};
}
