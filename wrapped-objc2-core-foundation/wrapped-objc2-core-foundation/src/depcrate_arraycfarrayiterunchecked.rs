// Generated macro for CFArrayIterUnchecked (struct)
macro_rules! Depcrate_arrayCFArrayIterUnchecked {
() => {
// Module: crate::array
// Provides: {"CFArrayIterUnchecked"}
// Dependencies: {}
# [doc = " An iterator over raw items of an array."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The array must not be mutated while this is alive."] # [derive (Debug)] pub struct CFArrayIterUnchecked < 'a , T : ? Sized + 'a > { array : & 'a CFArray < T > , index : CFIndex , len : CFIndex , }
};
}
