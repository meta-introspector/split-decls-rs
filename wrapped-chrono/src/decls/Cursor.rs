macro_rules! Cursor {
    () => {
        # [doc = " A `Cursor` contains a slice of a buffer and a read count."] # [derive (Debug , Eq , PartialEq)] pub (crate) struct Cursor < 'a > { # [doc = " Slice representing the remaining data to be read"] remaining : & 'a [u8] , # [doc = " Number of already read bytes"] read_count : usize , }
    };
}

Cursor!();