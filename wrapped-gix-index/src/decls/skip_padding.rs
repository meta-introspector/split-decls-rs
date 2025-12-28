macro_rules! skip_padding {
    () => {
        # [inline] fn skip_padding (data : & [u8] , first_byte_of_entry : usize) -> & [u8] { let current_offset = data . as_ptr () as usize ; let c_padding = (current_offset - first_byte_of_entry + 8) & ! 7 ; let skip = (first_byte_of_entry + c_padding) - current_offset ; & data [skip ..] }
    };
}

skip_padding!()