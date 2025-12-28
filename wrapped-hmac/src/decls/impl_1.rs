macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < D : EagerHash > Clone for HmacCore < D > { fn clone (& self) -> Self { Self { digest : self . digest . clone () , opad_digest : self . opad_digest . clone () , } } }
    };
}

impl_1!();