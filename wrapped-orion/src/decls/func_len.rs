macro_rules! func_len {
    () => {
        # [doc = " Macro to implement a `len()` function which will return the original_length"] # [doc = " field. Meaning the amount of bytes the newtype was created from."] macro_rules ! func_len (() => (# [inline] # [doc = " Return the length of the object."] pub fn len (& self) -> usize { self . original_length })) ;
    };
}

func_len!()