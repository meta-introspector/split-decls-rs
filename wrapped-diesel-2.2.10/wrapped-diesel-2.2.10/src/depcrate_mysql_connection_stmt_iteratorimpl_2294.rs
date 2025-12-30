// Generated macro for impl_2294 (impl)
macro_rules! Depcrate_mysql_connection_stmt_iteratorimpl_2294 {
() => {
// Module: crate::mysql::connection::stmt::iterator
// Provides: {"impl_2294"}
// Dependencies: {}
impl < 'a > Row < 'a , Mysql > for MysqlRow { type Field < 'f > = MysqlField < 'f > where 'a : 'f , Self : 'f ; type InnerPartialRow = Self ; fn field_count (& self) -> usize { self . metadata . fields () . len () } fn get < 'b , I > (& 'b self , idx : I) -> Option < Self :: Field < 'b > > where 'a : 'b , Self : RowIndex < I > , { let idx = self . idx (idx) ? ; Some (MysqlField { binds : self . row . borrow () , metadata : self . metadata . clone () , idx , }) } fn partial_row (& self , range : std :: ops :: Range < usize >) -> PartialRow < '_ , Self :: InnerPartialRow > { PartialRow :: new (self , range) } }
};
}
