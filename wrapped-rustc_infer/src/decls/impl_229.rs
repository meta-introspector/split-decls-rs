macro_rules! deps {
    () => {
        TyVidSubKey!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl From < ty :: TyVid > for TyVidSubKey { # [inline] fn from (vid : ty :: TyVid) -> Self { TyVidSubKey { vid } } }
    };
}

impl_229!()