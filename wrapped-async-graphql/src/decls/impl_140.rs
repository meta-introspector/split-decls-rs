macro_rules! deps {
    () => {
        TestInput!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl Default for TestInput { fn default () -> Self { Self { id : 423 , name : "foo" . to_string () , } } }
    };
}

impl_140!()