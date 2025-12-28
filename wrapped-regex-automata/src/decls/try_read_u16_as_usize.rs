macro_rules! deps {
    () => {
        DeserializeError!();
    };
}

macro_rules! try_read_u16_as_usize {
    () => {
        deps!();
        # [doc = " Try to read a u16 as a usize from the beginning of the given slice in"] # [doc = " native endian format. If the slice has fewer than 2 bytes or if the"] # [doc = " deserialized number cannot be represented by usize, then this returns an"] # [doc = " error. The error message will include the `what` description of what is"] # [doc = " being deserialized, for better error messages. `what` should be a noun in"] # [doc = " singular form."] # [doc = ""] # [doc = " Upon success, this also returns the number of bytes read."] pub (crate) fn try_read_u16_as_usize (slice : & [u8] , what : & 'static str ,) -> Result < (usize , usize) , DeserializeError > { try_read_u16 (slice , what) . and_then (| (n , nr) | { usize :: try_from (n) . map (| n | (n , nr)) . map_err (| _ | DeserializeError :: invalid_usize (what)) }) }
    };
}

try_read_u16_as_usize!();