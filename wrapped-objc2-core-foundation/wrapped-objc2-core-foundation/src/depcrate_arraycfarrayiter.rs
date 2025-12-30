// Generated macro for CFArrayIter (struct)
macro_rules! Depcrate_arrayCFArrayIter {
() => {
// Module: crate::array
// Provides: {"CFArrayIter"}
// Dependencies: {}
# [doc = " An iterator over retained objects of an array."] # [derive (Debug)] pub struct CFArrayIter < 'a , T : ? Sized + 'a > { array : & 'a CFArray < T > , index : usize , }
};
}
