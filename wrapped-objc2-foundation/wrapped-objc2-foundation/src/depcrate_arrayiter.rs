// Generated macro for Iter (struct)
macro_rules! Depcrate_arrayIter {
() => {
// Module: crate::array
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the items of an array."] # [derive (Debug)] # [cfg (feature = "NSEnumerator")] pub struct Iter < 'a , ObjectType : Message > (iter :: Iter < 'a , NSArray < ObjectType > >) ;
};
}
