// Generated macro for CFArrayIntoIter (struct)
macro_rules! Depcrate_arrayCFArrayIntoIter {
() => {
// Module: crate::array
// Provides: {"CFArrayIntoIter"}
// Dependencies: {}
# [doc = " A retained iterator over the items of an array."] # [derive (Debug)] pub struct CFArrayIntoIter < T : ? Sized > { array : CFRetained < CFArray < T > > , index : usize , }
};
}
