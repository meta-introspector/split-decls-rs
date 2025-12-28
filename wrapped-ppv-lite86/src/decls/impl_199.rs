macro_rules! deps {
    () => {
        MultiLane!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < S3 , S4 , NI > Debug for u32x4_sse2 < S3 , S4 , NI > where Self : Copy + MultiLane < [u32 ; 4] > , { # [cold] fn fmt (& self , fmt : & mut Formatter) -> Result { fmt . write_fmt (format_args ! ("{:08x?}" , & self . to_lanes ())) } }
    };
}

impl_199!();