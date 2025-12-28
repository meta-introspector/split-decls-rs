macro_rules! deps {
    () => {
        OptionsMustOutliveDB!();
        BlockBasedOptionsMustOutliveDB!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl OptionsMustOutliveDB { pub (crate) fn clone (& self) -> Self { Self { env : self . env . clone () , row_cache : self . row_cache . clone () , blob_cache : self . blob_cache . clone () , block_based : self . block_based . as_ref () . map (BlockBasedOptionsMustOutliveDB :: clone) , write_buffer_manager : self . write_buffer_manager . clone () , } } }
    };
}

impl_152!();