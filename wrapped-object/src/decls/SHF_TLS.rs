macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_TLS {
    () => {
        deps!();
        # [doc = " Section holds thread-local storage."] pub const SHF_TLS : u32 = 1 << 10 ;
    };
}

SHF_TLS!()