// Generated macro for impl_739 (impl)
macro_rules! Depcrate_insertableimpl_739 {
() => {
// Module: crate::insertable
// Provides: {"impl_739"}
// Dependencies: {}
impl < 'a , T , Tab , const N : usize > Insertable < Tab > for & 'a [T ; N] where T : Insertable < Tab > , & 'a T : Insertable < Tab > , { type Values = BatchInsert < Vec < < & 'a T as Insertable < Tab > > :: Values > , Tab , [T :: Values ; N] , true > ; fn values (self) -> Self :: Values { let values = self . iter () . map (Insertable :: values) . collect () ; BatchInsert :: new (values) } }
};
}
