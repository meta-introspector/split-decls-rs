macro_rules! deps {
    () => {
        OpenTelemetry!();
    };
}

macro_rules! impl_559 {
    () => {
        deps!();
        impl < T > OpenTelemetry < T > { # [doc = " Use `tracer` to create an OpenTelemetry extension."] pub fn new (tracer : T) -> OpenTelemetry < T > where T : Tracer + Send + Sync + 'static , < T as Tracer > :: Span : Sync + Send , { Self { tracer : Arc :: new (tracer) , } } }
    };
}

impl_559!();