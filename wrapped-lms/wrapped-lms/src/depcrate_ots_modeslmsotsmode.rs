// Generated macro for LmsOtsMode (trait)
macro_rules! Depcrate_ots_modesLmsOtsMode {
() => {
// Module: crate::ots::modes
// Provides: {"LmsOtsMode"}
// Dependencies: {}
# [doc = " The basic trait that must be implemented by any OTS mode."] pub trait LmsOtsMode : Typecode { # [doc = " The underlying hash function"] type Hasher : Digest ; # [doc = " The length of the hash function output as a type"] type NLen : ArraySize ; # [doc = " The value of P as a type"] type PLen : ArraySize ; # [doc = " The length of the hash function output as a [usize]"] const N : usize ; # [doc = " The Winternitz window, which should be a value that divides 8"] const W : usize ; # [doc = " The number of `W` bit fields required to contain the hash of the message"] const U : usize ; # [doc = " The number of `W` bit fields required to contain the checksum"] const V : usize ; # [doc = " Computed as `U` + `V`"] const P : usize ; # [doc = " The left shift required to get the checksum bits"] const LS : usize ; # [doc = " The total length of the signature"] const SIG_LEN : usize ; # [doc = " Expands a message into its Winternitz coefficients and checksum"] fn expand (message : & Output < Self :: Hasher >) -> Array < u8 , Self :: PLen > { let mut arr : Array < u8 , < Self as LmsOtsMode > :: PLen > = Array :: default () ; for (i , c) in coefs (message , Self :: W) . enumerate () . take (Self :: U) { arr [i] = c ; } let cksum = (& arr) . into_iter () . take (Self :: U) . map (| & x | (1u16 << Self :: W) - 1 - (x as u16)) . sum :: < u16 > () << Self :: LS ; let cksum_bytes = cksum . to_be_bytes () ; let cksum_chunks = coefs (& cksum_bytes , Self :: W) . take (Self :: V) ; for (i , c) in cksum_chunks . enumerate () { arr [Self :: U + i] = c ; } arr } }
};
}
