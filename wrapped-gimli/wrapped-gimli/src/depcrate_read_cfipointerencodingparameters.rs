// Generated macro for PointerEncodingParameters (struct)
macro_rules! Depcrate_read_cfiPointerEncodingParameters {
() => {
// Module: crate::read::cfi
// Provides: {"PointerEncodingParameters"}
// Dependencies: {}
# [derive (Clone , Debug)] struct PointerEncodingParameters < 'a , R : Reader > { bases : & 'a SectionBaseAddresses , func_base : Option < u64 > , address_size : u8 , section : & 'a R , }
};
}
