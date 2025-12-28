macro_rules! deps {
    () => {
        DeserializeError!();
    };
}

macro_rules! try_read_u32 {
    () => {
        deps!();
        # [doc = " Try to read a u32 from the beginning of the given slice in native endian"] # [doc = " format. If the slice has fewer than 4 bytes, then this returns an error."] # [doc = " The error message will include the `what` description of what is being"] # [doc = " deserialized, for better error messages. `what` should be a noun in"] # [doc = " singular form."] # [doc = ""] # [doc = " Upon success, this also returns the number of bytes read."] pub (crate) fn try_read_u32 (slice : & [u8] , what : & 'static str ,) -> Result < (u32 , usize) , DeserializeError > { check_slice_len (slice , size_of :: < u32 > () , what) ? ; Ok ((read_u32 (slice) , size_of :: < u32 > ())) }
    };
}

try_read_u32!()