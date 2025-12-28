macro_rules! deps {
    () => {
        ExtensionFactory!();
        Extension!();
        OpenTelemetry!();
        OpenTelemetryExtension!();
    };
}

macro_rules! impl_560 {
    () => {
        deps!();
        impl < T > ExtensionFactory for OpenTelemetry < T > where T : Tracer + Send + Sync + 'static , < T as Tracer > :: Span : Sync + Send , { fn create (& self) -> Arc < dyn Extension > { Arc :: new (OpenTelemetryExtension { tracer : self . tracer . clone () , }) } }
    };
}

impl_560!()