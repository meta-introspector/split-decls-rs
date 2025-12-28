macro_rules! SIGN_BIT {
    () => {
        # [cfg (feature = "read-core")] const SIGN_BIT : u8 = 1 << 6 ;
    };
}

SIGN_BIT!()