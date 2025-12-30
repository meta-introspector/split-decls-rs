// Generated macro for print_gnu_hash (function)
macro_rules! Depcrate_readobj_elfprint_gnu_hash {
() => {
// Module: crate::readobj::elf
// Provides: {"print_gnu_hash"}
// Dependencies: {}
fn print_gnu_hash < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , _elf : & Elf , _sections : & SectionTable < Elf > , section : & Elf :: SectionHeader ,) { if let Some (Some (hash)) = section . gnu_hash_header (endian , data) . print_err (p) { p . group ("GnuHash" , | p | { p . field ("BucketCount" , hash . bucket_count . get (endian)) ; p . field ("SymbolBase" , hash . symbol_base . get (endian)) ; p . field ("BloomCount" , hash . bloom_count . get (endian)) ; p . field ("BloomShift" , hash . bloom_shift . get (endian)) ; }) ; } }
};
}
