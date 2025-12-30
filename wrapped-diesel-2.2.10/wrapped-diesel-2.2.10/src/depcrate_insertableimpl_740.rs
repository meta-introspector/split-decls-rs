// Generated macro for impl_740 (impl)
macro_rules! Depcrate_insertableimpl_740 {
() => {
// Module: crate::insertable
// Provides: {"impl_740"}
// Dependencies: {}
impl < T , Tab , const N : usize > Insertable < Tab > for Box < [T ; N] > where T : Insertable < Tab > , { type Values = BatchInsert < Vec < T :: Values > , Tab , [T :: Values ; N] , true > ; fn values (self) -> Self :: Values { let v = Vec :: from (self as Box < [T] >) ; let values = v . into_iter () . map (Insertable :: values) . collect :: < Vec < _ > > () ; BatchInsert :: new (values) } }
};
}
