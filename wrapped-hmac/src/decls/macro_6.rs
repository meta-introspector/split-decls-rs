macro_rules! macro_6 {
    () => {
        digest :: buffer_fixed ! (# [doc = " Generic HMAC instance with reset support."] pub struct HmacReset < D : EagerHash > (block_api :: HmacResetCore < D >) ; impl : ResetMacTraits KeyInit ;) ;
    };
}

macro_6!()