// Generated macro for Iter (struct)
macro_rules! Depcrate_setIter {
() => {
// Module: crate::set
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the items of a set."] # [derive (Debug)] # [cfg (feature = "NSEnumerator")] pub struct Iter < 'a , ObjectType : Message > (iter :: Iter < 'a , NSSet < ObjectType > >) ;
};
}
