// Generated macro for impl_747 (impl)
macro_rules! Depcrate_insertableimpl_747 {
() => {
// Module: crate::insertable
// Provides: {"impl_747"}
// Dependencies: {}
impl < 'a , L , R , Tab > Insertable < Tab > for & 'a Grouped < crate :: expression :: operators :: Eq < L , R > > where & 'a crate :: expression :: operators :: Eq < L , R > : Insertable < Tab > , { type Values = < & 'a crate :: expression :: operators :: Eq < L , R > as Insertable < Tab > > :: Values ; fn values (self) -> Self :: Values { self . 0 . values () } }
};
}
