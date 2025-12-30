// Generated macro for impl_745 (impl)
macro_rules! Depcrate_insertableimpl_745 {
() => {
// Module: crate::insertable
// Provides: {"impl_745"}
// Dependencies: {}
impl < 'a , T , Tab > Insertable < Tab > for & 'a Option < T > where Option < & 'a T > : Insertable < Tab > , { type Values = < Option < & 'a T > as Insertable < Tab > > :: Values ; fn values (self) -> Self :: Values { self . as_ref () . values () } }
};
}
