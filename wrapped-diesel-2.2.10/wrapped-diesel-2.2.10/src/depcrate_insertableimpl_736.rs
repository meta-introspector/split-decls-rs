// Generated macro for impl_736 (impl)
macro_rules! Depcrate_insertableimpl_736 {
() => {
// Module: crate::insertable
// Provides: {"impl_736"}
// Dependencies: {}
impl < 'a , T , Tab > Insertable < Tab > for & 'a Vec < T > where & 'a [T] : Insertable < Tab > , { type Values = < & 'a [T] as Insertable < Tab > > :: Values ; fn values (self) -> Self :: Values { (& * * self) . values () } }
};
}
