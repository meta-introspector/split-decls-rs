macro_rules! RawBytesNote {
    () => {
        # [derive (Subdiagnostic)] # [note (const_eval_raw_bytes)] pub struct RawBytesNote { pub size : u64 , pub align : u64 , pub bytes : String , }
    };
}

RawBytesNote!();