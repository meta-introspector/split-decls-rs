// Generated macro for EncryptionInfoCommand64 (struct)
macro_rules! Depcrate_machoEncryptionInfoCommand64 {
() => {
// Module: crate::macho
// Provides: {"EncryptionInfoCommand64"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct EncryptionInfoCommand64 < E : Endian > { # [doc = " LC_ENCRYPTION_INFO_64"] pub cmd : U32 < E > , # [doc = " sizeof(struct EncryptionInfoCommand64)"] pub cmdsize : U32 < E > , # [doc = " file offset of encrypted range"] pub cryptoff : U32 < E > , # [doc = " file size of encrypted range"] pub cryptsize : U32 < E > , # [doc = " which enryption system, 0 means not-encrypted yet"] pub cryptid : U32 < E > , # [doc = " padding to make this struct's size a multiple of 8 bytes"] pub pad : U32 < E > , }
};
}
