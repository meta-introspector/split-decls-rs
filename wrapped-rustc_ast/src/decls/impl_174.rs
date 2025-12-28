macro_rules! deps {
    () => {
        BoundConstness!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl BoundConstness { pub fn as_str (self) -> & 'static str { match self { Self :: Never => "" , Self :: Always (_) => "const" , Self :: Maybe (_) => "[const]" , } } }
    };
}

impl_174!()