macro_rules! deps {
    () => {
        RemoteRedirect!();
    };
}

macro_rules! impl_661 {
    () => {
        deps!();
        impl Default for RemoteRedirect { fn default () -> Self { RemoteRedirect :: Initial } }
    };
}

impl_661!()