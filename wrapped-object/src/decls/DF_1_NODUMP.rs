macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! DF_1_NODUMP {
    () => {
        deps!();
        # [doc = " Object can't be dldump'ed."] pub const DF_1_NODUMP : u32 = 0x0000_1000 ;
    };
}

DF_1_NODUMP!();