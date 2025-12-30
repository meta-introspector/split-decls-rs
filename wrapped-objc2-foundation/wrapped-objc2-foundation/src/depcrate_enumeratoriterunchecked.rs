// Generated macro for IterUnchecked (struct)
macro_rules! Depcrate_enumeratorIterUnchecked {
() => {
// Module: crate::enumerator
// Provides: {"IterUnchecked"}
// Dependencies: {}
# [doc = " An iterator over unretained items in an enumerator."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The enumerator and the underlying collection must not be mutated while"] # [doc = " this is alive."] # [derive (Debug)] pub struct IterUnchecked < 'a , ObjectType : Message + 'a > (iter :: IterUnchecked < 'a , NSEnumerator < ObjectType > > ,) ;
};
}
