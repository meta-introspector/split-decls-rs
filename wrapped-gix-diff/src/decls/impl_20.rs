macro_rules! deps {
    () => {
        Copies!();
        CopySource!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Default for Copies { fn default () -> Self { Copies { source : CopySource :: default () , percentage : Some (0.5) , } } }
    };
}

impl_20!()