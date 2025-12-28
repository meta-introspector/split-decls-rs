macro_rules! deps {
    () => {
        OutputBuffer!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < 'a > OutputBuffer < 'a > { # [inline] pub fn from_slice_pos_and_max (slice : & 'a mut [u8] , position : usize , max_count : usize ,) -> OutputBuffer < 'a > { let mut max = position . saturating_add (max_count) ; if max > slice . len () { max = slice . len () ; } OutputBuffer { slice , position , max , } } # [inline (always)] pub const fn position (& self) -> usize { self . position } # [inline (always)] pub fn set_position (& mut self , position : usize) { self . position = position ; } # [doc = " Write a byte to the current position and increment"] # [doc = ""] # [doc = " Assumes that there is space."] # [inline] pub fn write_byte (& mut self , byte : u8) { self . slice [self . position] = byte ; self . position += 1 ; } # [doc = " Write a slice to the current position and increment"] # [doc = ""] # [doc = " Assumes that there is space."] # [inline] pub fn write_slice (& mut self , data : & [u8]) { let len = data . len () ; self . slice [self . position .. self . position + len] . copy_from_slice (data) ; self . position += data . len () ; } # [inline] pub const fn bytes_left (& self) -> usize { self . max - self . position } # [inline (always)] pub const fn get_ref (& self) -> & [u8] { self . slice } # [inline (always)] pub fn get_mut (& mut self) -> & mut [u8] { self . slice } }
    };
}

impl_176!();