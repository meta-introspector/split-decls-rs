// Generated macro for ProjectionElem (enum)
macro_rules! Depcrate_mirProjectionElem {
() => {
// Module: crate::mir
// Provides: {"ProjectionElem"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum ProjectionElem < V , T > { Deref , Field (Either < FieldId , TupleFieldId >) , ClosureField (usize) , Index (V) , ConstantIndex { offset : u64 , from_end : bool } , Subslice { from : u64 , to : u64 } , OpaqueCast (T) , }
};
}
