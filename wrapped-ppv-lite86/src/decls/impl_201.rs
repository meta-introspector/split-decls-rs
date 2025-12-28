macro_rules! deps {
    () => {
        MultiLane!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < S3 , S4 , NI > Debug for u64x2_sse2 < S3 , S4 , NI > where Self : Copy + MultiLane < [u64 ; 2] > , { # [cold] fn fmt (& self , fmt : & mut Formatter) -> Result { fmt . write_fmt (format_args ! ("{:016x?}" , & self . to_lanes ())) } }
    };
}

impl_201!()