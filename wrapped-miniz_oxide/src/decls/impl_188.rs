macro_rules! deps {
    () => {
        InflateState!();
        TINFLStatus!();
        DataFormat!();
        DecompressorOxide!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl Default for InflateState { fn default () -> Self { InflateState { decomp : DecompressorOxide :: default () , dict : [0 ; TINFL_LZ_DICT_SIZE] , dict_ofs : 0 , dict_avail : 0 , first_call : true , has_flushed : false , data_format : DataFormat :: Raw , last_status : TINFLStatus :: NeedsMoreInput , } } }
    };
}

impl_188!()