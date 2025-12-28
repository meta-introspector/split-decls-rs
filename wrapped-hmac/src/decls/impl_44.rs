macro_rules! deps {
    () => {
        SimpleHmacReset!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser + FixedOutputReset > FixedOutputReset for SimpleHmacReset < D > { fn finalize_into_reset (& mut self , out : & mut Output < Self >) { let mut h = D :: new () ; Update :: update (& mut h , & self . opad_key) ; Update :: update (& mut h , & self . digest . finalize_reset ()) ; Update :: update (& mut self . digest , & self . ipad_key) ; Digest :: finalize_into (h , out) ; } }
    };
}

impl_44!()