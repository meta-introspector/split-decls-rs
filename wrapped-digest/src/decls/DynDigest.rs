macro_rules! deps {
    () => {
        InvalidBufferSize!();
        Digest!();
    };
}

macro_rules! DynDigest {
    () => {
        deps!();
        # [doc = " Modification of the [`Digest`] trait suitable for trait objects."] pub trait DynDigest { # [doc = " Digest input data."] # [doc = ""] # [doc = " This method can be called repeatedly for use with streaming messages."] fn update (& mut self , data : & [u8]) ; # [doc = " Retrieve result and reset hasher instance"] # [cfg (feature = "alloc")] fn finalize_reset (& mut self) -> Box < [u8] > { let mut result = vec ! [0 ; self . output_size ()] ; self . finalize_into_reset (& mut result) . unwrap () ; result . into_boxed_slice () } # [doc = " Retrieve result and consume boxed hasher instance"] # [cfg (feature = "alloc")] # [allow (clippy :: boxed_local)] fn finalize (mut self : Box < Self >) -> Box < [u8] > { let mut result = vec ! [0 ; self . output_size ()] ; self . finalize_into_reset (& mut result) . unwrap () ; result . into_boxed_slice () } # [doc = " Write result into provided array and consume the hasher instance."] # [doc = ""] # [doc = " Returns error if buffer length is not equal to `output_size`."] fn finalize_into (self , buf : & mut [u8]) -> Result < () , InvalidBufferSize > ; # [doc = " Write result into provided array and reset the hasher instance."] # [doc = ""] # [doc = " Returns error if buffer length is not equal to `output_size`."] fn finalize_into_reset (& mut self , out : & mut [u8]) -> Result < () , InvalidBufferSize > ; # [doc = " Reset hasher instance to its initial state."] fn reset (& mut self) ; # [doc = " Get output size of the hasher"] fn output_size (& self) -> usize ; # [doc = " Clone hasher state into a boxed trait object"] # [cfg (feature = "alloc")] fn box_clone (& self) -> Box < dyn DynDigest > ; }
    };
}

DynDigest!()