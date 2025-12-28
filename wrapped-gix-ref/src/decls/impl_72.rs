macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < ObjectId > for Target { fn from (id : ObjectId) -> Self { Target :: Object (id) } }
    };
}

impl_72!()