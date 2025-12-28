macro_rules! deps {
    () => {
        ExtensionFactory!();
        Extension!();
        LoggerExtension!();
        Logger!();
    };
}

macro_rules! impl_547 {
    () => {
        deps!();
        impl ExtensionFactory for Logger { fn create (& self) -> Arc < dyn Extension > { Arc :: new (LoggerExtension) } }
    };
}

impl_547!()