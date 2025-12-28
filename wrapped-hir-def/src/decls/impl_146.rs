macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl fmt :: Debug for RawVisibilityId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_tuple ("RawVisibilityId") ; match * self { Self :: PUB => f . field (& "pub") , Self :: PRIV_IMPLICIT | Self :: PRIV_EXPLICIT => f . field (& "pub(self)") , Self :: PUB_CRATE => f . field (& "pub(crate)") , _ => f . field (& self . 0) , } ; f . finish () } }
    };
}

impl_146!()