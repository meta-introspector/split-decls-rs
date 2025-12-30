// Generated macro for RtlInitHashTableContextFromEnumerator (function)
macro_rules! Depcrate_ntrtlRtlInitHashTableContextFromEnumerator {
() => {
// Module: crate::ntrtl
// Provides: {"RtlInitHashTableContextFromEnumerator"}
// Dependencies: {}
# [inline] pub fn RtlInitHashTableContextFromEnumerator (Context : & mut RTL_DYNAMIC_HASH_TABLE_CONTEXT , Enumerator : & RTL_DYNAMIC_HASH_TABLE_ENUMERATOR ,) { Context . ChainHead = Enumerator . ChainHead ; Context . PrevLinkage = Enumerator . HashEntry . Linkage . Blink ; }
};
}
