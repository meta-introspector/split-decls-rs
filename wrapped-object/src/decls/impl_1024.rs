macro_rules! deps {
    () => {
        WritableBuffer!();
        Pod!();
    };
}

macro_rules! impl_1024 {
    () => {
        deps!();
        impl < 'a > dyn WritableBuffer + 'a { # [doc = " Writes the specified `Pod` type at the end of the buffer."] pub fn write < T : Pod > (& mut self , val : & T) { self . write_bytes (bytes_of (val)) } # [doc = " Writes the specified `Pod` slice at the end of the buffer."] pub fn write_slice < T : Pod > (& mut self , val : & [T]) { self . write_bytes (bytes_of_slice (val)) } }
    };
}

impl_1024!()