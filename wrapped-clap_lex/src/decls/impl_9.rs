macro_rules! deps {
    () => {
        RawArgs!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < I , T > From < I > for RawArgs where I : Iterator < Item = T > , T : Into < OsString > , { fn from (val : I) -> Self { Self { items : val . map (| x | x . into ()) . collect () , } } }
    };
}

impl_9!()