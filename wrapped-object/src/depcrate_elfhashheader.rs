// Generated macro for HashHeader (struct)
macro_rules! Depcrate_elfHashHeader {
() => {
// Module: crate::elf
// Provides: {"HashHeader"}
// Dependencies: {}
# [doc = " Header of `SHT_HASH` section."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct HashHeader < E : Endian > { # [doc = " The number of hash buckets."] pub bucket_count : U32 < E > , # [doc = " The number of chain values."] pub chain_count : U32 < E > , }
};
}
