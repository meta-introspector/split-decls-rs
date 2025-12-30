// Generated macro for IterUnchecked (struct)
macro_rules! Depcrate_arrayIterUnchecked {
() => {
// Module: crate::array
// Provides: {"IterUnchecked"}
// Dependencies: {}
# [doc = " An iterator over unretained items of an array."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The array must not be mutated while this is alive."] # [derive (Debug)] # [cfg (feature = "NSEnumerator")] pub struct IterUnchecked < 'a , ObjectType : Message > (iter :: IterUnchecked < 'a , NSArray < ObjectType > >) ;
};
}
