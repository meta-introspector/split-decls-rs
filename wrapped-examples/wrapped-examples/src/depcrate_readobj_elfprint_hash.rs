// Generated macro for print_hash (function)
macro_rules! Depcrate_readobj_elfprint_hash {
() => {
// Module: crate::readobj::elf
// Provides: {"print_hash"}
// Dependencies: {}
fn print_hash < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , _elf : & Elf , _sections : & SectionTable < Elf > , section : & Elf :: SectionHeader ,) { if let Some (Some (hash)) = section . hash_header (endian , data) . print_err (p) { p . group ("Hash" , | p | { p . field ("BucketCount" , hash . bucket_count . get (endian)) ; p . field ("ChainCount" , hash . chain_count . get (endian)) ; }) ; } }
};
}
