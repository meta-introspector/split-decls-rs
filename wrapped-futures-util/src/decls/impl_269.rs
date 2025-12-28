macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < T > Either < T , T > { # [doc = " Extract the value of an either over two equivalent types."] pub fn into_inner (self) -> T { match self { Self :: Left (x) | Self :: Right (x) => x , } } }
    };
}

impl_269!();