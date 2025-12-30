// Generated macro for MappedGenericSequence (trait)
macro_rules! Depcrate_functionalMappedGenericSequence {
() => {
// Module: crate::functional
// Provides: {"MappedGenericSequence"}
// Dependencies: {}
# [doc = " Defines the relationship between one generic sequence and another,"] # [doc = " for operations such as `map` and `zip`."] pub trait MappedGenericSequence < T , U > : GenericSequence < T > { # [doc = " Mapped sequence type"] type Mapped : GenericSequence < U , Length = Self :: Length > ; }
};
}
