// Generated macro for AddSerializedStateSize (type)
macro_rules! Depcrate_hazmatAddSerializedStateSize {
() => {
// Module: crate::hazmat
// Provides: {"AddSerializedStateSize"}
// Dependencies: {}
# [doc = " Alias for `AddSerializedStateSize<T, S> = Sum<T, S::SerializedStateSize>`"] pub type AddSerializedStateSize < T , S > = Sum < T , < S as SerializableState > :: SerializedStateSize > ;
};
}
