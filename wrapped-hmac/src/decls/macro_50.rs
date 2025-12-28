macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! macro_50 {
    () => {
        deps!();
        digest :: buffer_fixed ! (# [doc = " Generic HMAC instance."] pub struct Hmac < D : EagerHash > (block_api :: HmacCore < D >) ; impl : MacTraits KeyInit ;) ;
    };
}

macro_50!();