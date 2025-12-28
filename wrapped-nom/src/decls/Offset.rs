macro_rules! Offset {
    () => {
        # [doc = " Useful functions to calculate the offset between slices and show a hexdump of a slice"] pub trait Offset { # [doc = " Offset between the first byte of self and the first byte of the argument"] # [doc = " the argument must be a part of self, otherwise this can fail with arithmetic"] # [doc = " underflows as it compares byte offsets"] fn offset (& self , second : & Self) -> usize ; }
    };
}

Offset!()