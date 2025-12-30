// Generated macro for impl_3264 (impl)
macro_rules! Depcrate_pg_connection_rowimpl_3264 {
() => {
// Module: crate::pg::connection::row
// Provides: {"impl_3264"}
// Dependencies: {}
impl < 'a > Row < 'a , Pg > for PgRow { type Field < 'f > = PgField < 'f > where 'a : 'f , Self : 'f ; type InnerPartialRow = Self ; fn field_count (& self) -> usize { self . db_result . column_count () } fn get < 'b , I > (& 'b self , idx : I) -> Option < Self :: Field < 'b > > where 'a : 'b , Self : RowIndex < I > , { let idx = self . idx (idx) ? ; Some (PgField { db_result : & self . db_result , row_idx : self . row_idx , col_idx : idx , }) } fn partial_row (& self , range : std :: ops :: Range < usize >) -> PartialRow < '_ , Self :: InnerPartialRow > { PartialRow :: new (self , range) } }
};
}
