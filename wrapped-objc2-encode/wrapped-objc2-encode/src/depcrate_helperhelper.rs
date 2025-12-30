// Generated macro for Helper (enum)
macro_rules! Depcrate_helperHelper {
() => {
// Module: crate::helper
// Provides: {"Helper"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , Hash)] # [non_exhaustive] pub (crate) enum Helper < 'a , E = Encoding > { Primitive (Primitive) , BitField (u8 , Option < & 'a (u64 , E) >) , Indirection (IndirectionKind , & 'a E) , Array (u64 , & 'a E) , Container (ContainerKind , & 'a str , & 'a [E]) , NoneInvalid , }
};
}
