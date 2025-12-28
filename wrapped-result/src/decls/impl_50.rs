macro_rules! deps {
    () => {
        HeapString!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Default for HeapString { fn default () -> Self { Self (core :: ptr :: null_mut ()) } }
    };
}

impl_50!()