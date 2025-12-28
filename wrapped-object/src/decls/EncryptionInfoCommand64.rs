macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! EncryptionInfoCommand64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct EncryptionInfoCommand64 < E : Endian > { # [doc = " LC_ENCRYPTION_INFO_64"] pub cmd : U32 < E > , # [doc = " sizeof(struct EncryptionInfoCommand64)"] pub cmdsize : U32 < E > , # [doc = " file offset of encrypted range"] pub cryptoff : U32 < E > , # [doc = " file size of encrypted range"] pub cryptsize : U32 < E > , # [doc = " which enryption system, 0 means not-encrypted yet"] pub cryptid : U32 < E > , # [doc = " padding to make this struct's size a multiple of 8 bytes"] pub pad : U32 < E > , }
    };
}

EncryptionInfoCommand64!();