macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! EncryptionInfoCommand32 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct EncryptionInfoCommand32 < E : Endian > { # [doc = " LC_ENCRYPTION_INFO"] pub cmd : U32 < E > , # [doc = " sizeof(struct EncryptionInfoCommand32)"] pub cmdsize : U32 < E > , # [doc = " file offset of encrypted range"] pub cryptoff : U32 < E > , # [doc = " file size of encrypted range"] pub cryptsize : U32 < E > , # [doc = " which enryption system, 0 means not-encrypted yet"] pub cryptid : U32 < E > , }
    };
}

EncryptionInfoCommand32!()