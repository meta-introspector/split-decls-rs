macro_rules! deps {
    () => {
        HmacHashFunction!();
    };
}

macro_rules! Hmac {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct Hmac < S : HmacHashFunction , const BLOCKSIZE : usize > { working_hasher : S , opad_hasher : S , ipad_hasher : S , is_finalized : bool , }
    };
}

Hmac!()