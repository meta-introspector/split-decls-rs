// Generated macro for impl_21 (impl)
macro_rules! Depcrate_astimpl_21 {
() => {
// Module: crate::ast
// Provides: {"impl_21"}
// Dependencies: {}
impl Display for ContainerKind { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (match self { ContainerKind :: Struct => "struct" , ContainerKind :: TupleStruct => "tuple struct" , ContainerKind :: UnitStruct => "unit struct" , ContainerKind :: StructVariant => "struct variant" , ContainerKind :: TupleVariant => "tuple variant" , ContainerKind :: UnitVariant => "unit variant" , }) } }
};
}
