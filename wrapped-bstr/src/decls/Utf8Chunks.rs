macro_rules! Utf8Chunks {
    () => {
        # [doc = " An iterator over chunks of valid UTF-8 in a byte slice."] # [doc = ""] # [doc = " See [`utf8_chunks`](trait.ByteSlice.html#method.utf8_chunks)."] # [derive (Clone , Debug)] pub struct Utf8Chunks < 'a > { pub (super) bytes : & 'a [u8] , }
    };
}

Utf8Chunks!()