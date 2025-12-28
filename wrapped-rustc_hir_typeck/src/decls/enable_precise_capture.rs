macro_rules! enable_precise_capture {
    () => {
        # [doc = " Precise capture is enabled if user is using Rust Edition 2021 or higher."] # [doc = " `span` is the span of the closure."] fn enable_precise_capture (span : Span) -> bool { span . at_least_rust_2021 () }
    };
}

enable_precise_capture!();