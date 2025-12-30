// Generated macro for impl_744 (impl)
macro_rules! Depcrate_insertableimpl_744 {
() => {
// Module: crate::insertable
// Provides: {"impl_744"}
// Dependencies: {}
impl < T , Tab , Expr , Col > Insertable < Tab > for InsertableOptionHelper < T , ColumnInsertValue < Col , Expr > > where T : Insertable < Tab , Values = ValuesClause < ColumnInsertValue < Col , Expr > , Tab > > , { type Values = ValuesClause < DefaultableColumnInsertValue < ColumnInsertValue < Col , Expr > > , Tab > ; fn values (self) -> Self :: Values { ValuesClause :: new (self . 0 . map (| v | DefaultableColumnInsertValue :: Expression (Insertable :: values (v) . values)) . unwrap_or_default () ,) } }
};
}
