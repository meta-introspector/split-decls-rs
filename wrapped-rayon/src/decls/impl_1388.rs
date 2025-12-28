macro_rules! deps {
    () => {
        SendPtr!();
    };
}

macro_rules! impl_1388 {
    () => {
        deps!();
        impl < T > SendPtr < T > { fn get (self) -> * mut T { self . 0 } }
    };
}

impl_1388!()