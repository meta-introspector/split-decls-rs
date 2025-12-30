// Generated macro for impl_1159 (impl)
macro_rules! Depcrate_scaffold_names_storageimpl_1159 {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_1159"}
// Dependencies: {}
impl < 'data , M : DynamicDataMarker , Variables > fmt :: Debug for DataPayloadWithVariablesBorrowed < 'data , M , Variables > where < M :: DataStruct as Yokeable < 'data > > :: Output : fmt :: Debug , Variables : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (core :: any :: type_name :: < Self > ()) . field ("inner" , & self . inner) . finish () } }
};
}
