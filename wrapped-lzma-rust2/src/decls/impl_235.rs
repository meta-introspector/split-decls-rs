macro_rules! deps {
    () => {
        RangeEncoderBuffer!();
        Write!();
        RangeEncoder!();
        Result!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl RangeEncoder < RangeEncoderBuffer > { pub (crate) fn write_to < W : Write > (& self , out : & mut W) -> crate :: Result < () > { self . inner . write_to (out) } pub (crate) fn finish_buffer (& mut self) -> crate :: Result < Option < usize > > { self . finish () ? ; Ok (Some (self . inner . pos)) } pub (crate) fn new_buffer (buf_size : usize) -> Self { Self :: new (RangeEncoderBuffer :: new (buf_size)) } pub (crate) fn reset_buffer (& mut self) { self . reset () ; self . inner . pos = 0 ; } # [inline] pub (crate) fn get_pending_size (& self) -> u32 { self . inner . pos as u32 + self . cache_size + 5 - 1 } }
    };
}

impl_235!();