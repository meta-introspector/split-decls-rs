macro_rules! deps {
    () => {
        Frame!();
    };
}

macro_rules! ResolveWhat {
    () => {
        deps!();
        pub enum ResolveWhat < 'a > { Address (* mut c_void) , Frame (& 'a Frame) , }
    };
}

ResolveWhat!();