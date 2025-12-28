macro_rules! deps {
    () => {
        ExtensionFactory!();
        Analyzer!();
        Extension!();
        AnalyzerExtension!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        impl ExtensionFactory for Analyzer { fn create (& self) -> Arc < dyn Extension > { Arc :: new (AnalyzerExtension :: default ()) } }
    };
}

impl_521!()