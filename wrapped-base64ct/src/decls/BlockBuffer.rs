macro_rules! BlockBuffer {
    () => {
        # [doc = " Base64 encode buffer for a 1-block output."] # [doc = ""] # [doc = " This handles a partial block of data, i.e. data which hasn't been"] # [derive (Clone , Default , Debug)] struct BlockBuffer { # [doc = " 3 decoded bytes to be encoded to a 4-byte Base64-encoded input."] bytes : [u8 ; Self :: SIZE] , # [doc = " Position within the buffer."] position : usize , }
    };
}

BlockBuffer!();