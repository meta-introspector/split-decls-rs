// Generated macro for ApproximateByteSet (struct)
macro_rules! Depcrate_arch_all_twowayApproximateByteSet {
() => {
// Module: crate::arch::all::twoway
// Provides: {"ApproximateByteSet"}
// Dependencies: {}
# [doc = " A bitset used to track whether a particular byte exists in a needle or not."] # [doc = ""] # [doc = " Namely, bit 'i' is set if and only if byte%64==i for any byte in the"] # [doc = " needle. If a particular byte in the haystack is NOT in this set, then one"] # [doc = " can conclude that it is also not in the needle, and thus, one can advance"] # [doc = " in the haystack by needle.len() bytes."] # [derive (Clone , Copy , Debug)] struct ApproximateByteSet (u64) ;
};
}
