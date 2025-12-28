macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! macro_52 {
    () => {
        deps!();
        digest :: buffer_fixed ! (# [doc = " Generic HMAC instance with reset support."] pub struct HmacReset < D : EagerHash > (block_api :: HmacResetCore < D >) ; impl : ResetMacTraits KeyInit ;) ;
    };
}

macro_52!()