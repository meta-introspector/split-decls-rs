// Generated macro for AggregateKind (enum)
macro_rules! Depcrate_mirAggregateKind {
() => {
// Module: crate::mir
// Provides: {"AggregateKind"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone)] pub enum AggregateKind < 'db > { # [doc = " The type is of the element"] Array (Ty < 'db >) , # [doc = " The type is of the tuple"] Tuple (Ty < 'db >) , Adt (VariantId , GenericArgs < 'db >) , Union (UnionId , FieldId) , Closure (Ty < 'db >) , }
};
}
