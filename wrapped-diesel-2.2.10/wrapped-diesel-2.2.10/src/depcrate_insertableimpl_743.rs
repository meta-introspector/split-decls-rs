// Generated macro for impl_743 (impl)
macro_rules! Depcrate_insertableimpl_743 {
() => {
// Module: crate::insertable
// Provides: {"impl_743"}
// Dependencies: {}
impl < T , Tab , V > Insertable < Tab > for Option < T > where T : Insertable < Tab , Values = ValuesClause < V , Tab > > , InsertableOptionHelper < T , V > : Insertable < Tab > , { type Values = < InsertableOptionHelper < T , V > as Insertable < Tab > > :: Values ; fn values (self) -> Self :: Values { InsertableOptionHelper (self , PhantomData) . values () } }
};
}
