macro_rules! StreamFooter {
    () => {
        # [derive (Debug)] struct StreamFooter { pub backward_size : u32 , pub stream_flags : [u8 ; 2] , }
    };
}

StreamFooter!()