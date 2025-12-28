macro_rules! deps {
    () => {
        ValueRange!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl From < std :: ops :: RangeFull > for ValueRange { fn from (_ : std :: ops :: RangeFull) -> Self { Self :: FULL } }
    };
}

impl_157!()