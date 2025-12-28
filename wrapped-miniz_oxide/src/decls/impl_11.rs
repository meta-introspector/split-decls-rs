macro_rules! deps {
    () => {
        HashBuffers!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Default for HashBuffers { fn default () -> HashBuffers { HashBuffers { dict : vec ! [0 ; LZ_DICT_FULL_SIZE] . into_boxed_slice () . try_into () . unwrap () , next : vec ! [0 ; LZ_DICT_SIZE] . into_boxed_slice () . try_into () . unwrap () , hash : vec ! [0 ; LZ_DICT_SIZE] . into_boxed_slice () . try_into () . unwrap () , } } }
    };
}

impl_11!()