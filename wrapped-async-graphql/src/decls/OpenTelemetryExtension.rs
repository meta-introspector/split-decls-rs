macro_rules! OpenTelemetryExtension {
    () => {
        struct OpenTelemetryExtension < T > { tracer : Arc < T > , }
    };
}

OpenTelemetryExtension!();