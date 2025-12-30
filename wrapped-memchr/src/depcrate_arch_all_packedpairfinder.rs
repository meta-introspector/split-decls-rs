// Generated macro for Finder (struct)
macro_rules! Depcrate_arch_all_packedpairFinder {
() => {
// Module: crate::arch::all::packedpair
// Provides: {"Finder"}
// Dependencies: {}
# [doc = " An architecture independent \"packed pair\" finder."] # [doc = ""] # [doc = " This finder picks two bytes that it believes have high predictive power for"] # [doc = " indicating an overall match of a needle. At search time, it reports offsets"] # [doc = " where the needle could match based on whether the pair of bytes it chose"] # [doc = " match."] # [doc = ""] # [doc = " This is architecture independent because it utilizes `memchr` to find the"] # [doc = " occurrence of one of the bytes in the pair, and then checks whether the"] # [doc = " second byte matches. If it does, in the case of [`Finder::find_prefilter`],"] # [doc = " the location at which the needle could match is returned."] # [doc = ""] # [doc = " It is generally preferred to use architecture specific routines for a"] # [doc = " \"packed pair\" prefilter, but this can be a useful fallback when the"] # [doc = " architecture independent routines are unavailable."] # [derive (Clone , Copy , Debug)] pub struct Finder { pair : Pair , byte1 : u8 , byte2 : u8 , }
};
}
