// Generated macro for impl_735 (impl)
macro_rules! Depcrate_insertableimpl_735 {
() => {
// Module: crate::insertable
// Provides: {"impl_735"}
// Dependencies: {}
impl < 'a , T , Tab > Insertable < Tab > for & 'a [T] where & 'a T : UndecoratedInsertRecord < Tab > + Insertable < Tab > , { type Values = BatchInsert < Vec < < & 'a T as Insertable < Tab > > :: Values > , Tab , () , false > ; fn values (self) -> Self :: Values { let values = self . iter () . map (Insertable :: values) . collect :: < Vec < _ > > () ; BatchInsert :: new (values) } }
};
}
