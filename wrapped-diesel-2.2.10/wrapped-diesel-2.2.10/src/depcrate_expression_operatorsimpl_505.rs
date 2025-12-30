// Generated macro for impl_505 (impl)
macro_rules! Depcrate_expression_operatorsimpl_505 {
() => {
// Module: crate::expression::operators
// Provides: {"impl_505"}
// Dependencies: {}
impl < 'a , T , Tab , U > Insertable < Tab > for & 'a Eq < T , U > where T : Copy , Eq < T , & 'a U > : Insertable < Tab > , { type Values = < Eq < T , & 'a U > as Insertable < Tab > > :: Values ; fn values (self) -> Self :: Values { Eq :: new (self . left , & self . right) . values () } }
};
}
