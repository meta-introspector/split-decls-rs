macro_rules! deps {
    () => {
        OutputBufferOxide!();
        CallbackOut!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl CallbackOut < '_ > { fn new_output_buffer < 'b > (& 'b mut self , local_buf : & 'b mut [u8] , out_buf_ofs : usize ,) -> OutputBufferOxide < 'b > { let is_local ; let buf_len = OUT_BUF_SIZE - 16 ; let chosen_buffer = match * self { CallbackOut :: Buf (ref mut cb) if cb . out_buf . len () - out_buf_ofs >= OUT_BUF_SIZE => { is_local = false ; & mut cb . out_buf [out_buf_ofs .. out_buf_ofs + buf_len] } _ => { is_local = true ; & mut local_buf [.. buf_len] } } ; OutputBufferOxide { inner : chosen_buffer , inner_pos : 0 , local : is_local , bit_buffer : 0 , bits_in : 0 , } } }
    };
}

impl_57!();