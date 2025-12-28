macro_rules! deps {
    () => {
        IntoStream!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl IntoStream for Vec < u8 > { fn into_stream (mut self) -> Self { self . resize (round (self . len () , 4) , 0) ; self } }
    };
}

impl_189!()