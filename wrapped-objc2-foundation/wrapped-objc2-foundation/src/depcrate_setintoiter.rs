// Generated macro for IntoIter (struct)
macro_rules! Depcrate_setIntoIter {
() => {
// Module: crate::set
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator over unretained items of a set."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The set must not be mutated while this is alive."] # [derive (Debug)] # [cfg (feature = "NSEnumerator")] pub struct IntoIter < ObjectType : Message > (iter :: IntoIter < NSSet < ObjectType > >) ;
};
}
