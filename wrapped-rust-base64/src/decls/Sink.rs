macro_rules! Sink {
    () => {
        # [doc = " The output mechanism for `ChunkedEncoder`'s encoded bytes."] pub trait Sink { type Error ; # [doc = " Handle a chunk of encoded base64 data (as UTF-8 bytes)"] fn write_encoded_bytes (& mut self , encoded : & [u8]) -> Result < () , Self :: Error > ; }
    };
}

Sink!()