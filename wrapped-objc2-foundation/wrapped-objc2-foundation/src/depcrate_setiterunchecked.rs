// Generated macro for IterUnchecked (struct)
macro_rules! Depcrate_setIterUnchecked {
() => {
// Module: crate::set
// Provides: {"IterUnchecked"}
// Dependencies: {}
# [doc = " An unchecked iterator over the items of a set."] # [derive (Debug)] # [cfg (feature = "NSEnumerator")] pub struct IterUnchecked < 'a , ObjectType : Message > (iter :: IterUnchecked < 'a , NSSet < ObjectType > >) ;
};
}
