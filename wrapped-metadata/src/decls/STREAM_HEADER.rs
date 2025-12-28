macro_rules! STREAM_HEADER {
    () => {
        # [repr (C)] struct STREAM_HEADER < const LEN : usize > { offset : u32 , size : u32 , name : [u8 ; LEN] , }
    };
}

STREAM_HEADER!()