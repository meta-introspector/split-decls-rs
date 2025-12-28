macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < D : EagerHash > Clone for HmacResetCore < D > { fn clone (& self) -> Self { Self { digest : self . digest . clone () , opad_digest : self . opad_digest . clone () , ipad_digest : self . ipad_digest . clone () , } } }
    };
}

impl_13!()