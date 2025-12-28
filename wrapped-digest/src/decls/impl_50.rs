macro_rules! deps {
    () => {
        FixedOutputReset!();
        DynDigest!();
        FixedOutput!();
        Update!();
        InvalidBufferSize!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < D : Update + FixedOutputReset + Reset + Clone + 'static > DynDigest for D { fn update (& mut self , data : & [u8]) { Update :: update (self , data) ; } # [cfg (feature = "alloc")] fn finalize_reset (& mut self) -> Box < [u8] > { FixedOutputReset :: finalize_fixed_reset (self) . to_vec () . into_boxed_slice () } # [cfg (feature = "alloc")] fn finalize (self : Box < Self >) -> Box < [u8] > { FixedOutput :: finalize_fixed (* self) . to_vec () . into_boxed_slice () } fn finalize_into (self , buf : & mut [u8]) -> Result < () , InvalidBufferSize > { buf . try_into () . map_err (| _ | InvalidBufferSize) . map (| buf | FixedOutput :: finalize_into (self , buf)) } fn finalize_into_reset (& mut self , buf : & mut [u8]) -> Result < () , InvalidBufferSize > { let buf = < & mut Output < Self > > :: try_from (buf) . map_err (| _ | InvalidBufferSize) ? ; FixedOutputReset :: finalize_into_reset (self , buf) ; Ok (()) } fn reset (& mut self) { Reset :: reset (self) ; } fn output_size (& self) -> usize { < Self as OutputSizeUser > :: OutputSize :: to_usize () } # [cfg (feature = "alloc")] fn box_clone (& self) -> Box < dyn DynDigest > { Box :: new (self . clone ()) } }
    };
}

impl_50!();