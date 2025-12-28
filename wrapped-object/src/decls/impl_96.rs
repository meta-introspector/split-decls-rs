macro_rules! deps {
    () => {
        ReadCacheOps!();
        Result!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < T : Read + Seek > ReadCacheOps for T { fn len (& mut self) -> Result < u64 , () > { self . seek (SeekFrom :: End (0)) . map_err (| _ | ()) } fn seek (& mut self , pos : u64) -> Result < u64 , () > { self . seek (SeekFrom :: Start (pos)) . map_err (| _ | ()) } fn read (& mut self , buf : & mut [u8]) -> Result < usize , () > { Read :: read (self , buf) . map_err (| _ | ()) } fn read_exact (& mut self , buf : & mut [u8]) -> Result < () , () > { Read :: read_exact (self , buf) . map_err (| _ | ()) } }
    };
}

impl_96!();