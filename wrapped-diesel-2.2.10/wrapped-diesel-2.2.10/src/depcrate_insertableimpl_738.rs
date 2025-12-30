// Generated macro for impl_738 (impl)
macro_rules! Depcrate_insertableimpl_738 {
() => {
// Module: crate::insertable
// Provides: {"impl_738"}
// Dependencies: {}
impl < T , Tab , const N : usize > Insertable < Tab > for [T ; N] where T : Insertable < Tab > , { type Values = BatchInsert < Vec < T :: Values > , Tab , [T :: Values ; N] , true > ; # [allow (deprecated)] fn values (self) -> Self :: Values { let values = std :: array :: IntoIter :: new (self) . map (Insertable :: values) . collect :: < Vec < _ > > () ; BatchInsert :: new (values) } }
};
}
