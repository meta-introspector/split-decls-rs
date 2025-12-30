// Generated macro for impl_737 (impl)
macro_rules! Depcrate_insertableimpl_737 {
() => {
// Module: crate::insertable
// Provides: {"impl_737"}
// Dependencies: {}
impl < T , Tab > Insertable < Tab > for Vec < T > where T : Insertable < Tab > + UndecoratedInsertRecord < Tab > , { type Values = BatchInsert < Vec < T :: Values > , Tab , () , false > ; fn values (self) -> Self :: Values { let values = self . into_iter () . map (Insertable :: values) . collect :: < Vec < _ > > () ; BatchInsert :: new (values) } }
};
}
