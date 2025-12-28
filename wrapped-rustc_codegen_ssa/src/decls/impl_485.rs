macro_rules! deps {
    () => {
        LocalRef!();
        Locals!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl < 'tcx , V > Index < mir :: Local > for Locals < 'tcx , V > { type Output = LocalRef < 'tcx , V > ; # [inline] fn index (& self , index : mir :: Local) -> & LocalRef < 'tcx , V > { & self . values [index] } }
    };
}

impl_485!()