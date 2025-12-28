macro_rules! deps {
    () => {
        Header!();
        MaskedRichHeaderEntry!();
    };
}

macro_rules! RichHeaderInfo {
    () => {
        deps!();
        # [doc = " Parsed information about a Rich Header."] # [derive (Debug , Clone , Copy)] pub struct RichHeaderInfo < 'data > { # [doc = " The offset at which the rich header starts."] pub offset : usize , # [doc = " The length (in bytes) of the rich header."] # [doc = ""] # [doc = " This includes the payload, but also the 16-byte start sequence and the"] # [doc = " 8-byte final \"Rich\" and XOR key."] pub length : usize , # [doc = " The XOR key used to mask the rich header."] # [doc = ""] # [doc = " Unless the file has been tampered with, it should be equal to a checksum"] # [doc = " of the file header."] pub xor_key : u32 , masked_entries : & 'data [pe :: MaskedRichHeaderEntry] , }
    };
}

RichHeaderInfo!()