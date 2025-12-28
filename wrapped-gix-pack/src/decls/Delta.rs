macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! Delta {
    () => {
        deps!();
        # [derive (Debug)] struct Delta { data : Range < usize > , base_size : usize , result_size : usize , decompressed_size : usize , data_offset : data :: Offset , }
    };
}

Delta!()