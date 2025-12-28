macro_rules! macro_4 {
    () => {
        digest :: buffer_fixed ! (# [doc = " Generic HMAC instance."] pub struct Hmac < D : EagerHash > (block_api :: HmacCore < D >) ; impl : MacTraits KeyInit ;) ;
    };
}

macro_4!()