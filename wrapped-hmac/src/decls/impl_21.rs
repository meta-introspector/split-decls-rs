macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < D : EagerHash > FixedOutputCore for HmacResetCore < D > { # [inline (always)] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let mut hash = Output :: < D :: Core > :: default () ; self . digest . finalize_fixed_core (buffer , & mut hash) ; buffer . reset () ; let mut h = self . opad_digest . clone () ; buffer . digest_blocks (& hash , | b | h . update_blocks (b)) ; h . finalize_fixed_core (buffer , out) ; } }
    };
}

impl_21!()