macro_rules! OpenTelemetry {
    () => {
        # [doc = " OpenTelemetry extension"] # [cfg_attr (docsrs , doc (cfg (feature = "opentelemetry")))] pub struct OpenTelemetry < T > { tracer : Arc < T > , }
    };
}

OpenTelemetry!();