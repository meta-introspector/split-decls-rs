// Generated macro for GenericArrayIter (struct)
macro_rules! Depcrate_iterGenericArrayIter {
() => {
// Module: crate::iter
// Provides: {"GenericArrayIter"}
// Dependencies: {}
# [doc = " An iterator that moves out of a [`GenericArray`]"] pub struct GenericArrayIter < T , N : ArrayLength > { array : ManuallyDrop < GenericArray < T , N > > , index : usize , index_back : usize , }
};
}
