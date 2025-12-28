macro_rules! deps {
    () => {
        Extension!();
        Tracing!();
        TracingExtension!();
        ExtensionFactory!();
    };
}

macro_rules! impl_565 {
    () => {
        deps!();
        impl ExtensionFactory for Tracing { fn create (& self) -> Arc < dyn Extension > { Arc :: new (TracingExtension) } }
    };
}

impl_565!();