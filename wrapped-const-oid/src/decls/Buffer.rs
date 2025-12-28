macro_rules! Buffer {
    () => {
        # [doc = " Array-backed buffer for storing BER computed at compile-time."] # [derive (Copy , Clone , Eq , Hash , PartialEq , PartialOrd , Ord)] pub struct Buffer < const SIZE : usize > { # [doc = " Length in bytes"] pub (crate) length : u8 , # [doc = " Array containing BER/DER-serialized bytes (no header)"] pub (crate) bytes : [u8 ; SIZE] , }
    };
}

Buffer!()