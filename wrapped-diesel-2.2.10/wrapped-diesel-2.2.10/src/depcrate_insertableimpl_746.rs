// Generated macro for impl_746 (impl)
macro_rules! Depcrate_insertableimpl_746 {
() => {
// Module: crate::insertable
// Provides: {"impl_746"}
// Dependencies: {}
impl < L , R , Tab > Insertable < Tab > for Grouped < crate :: expression :: operators :: Eq < L , R > > where crate :: expression :: operators :: Eq < L , R > : Insertable < Tab > , { type Values = < crate :: expression :: operators :: Eq < L , R > as Insertable < Tab > > :: Values ; fn values (self) -> Self :: Values { self . 0 . values () } }
};
}
