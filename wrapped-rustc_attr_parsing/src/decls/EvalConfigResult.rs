macro_rules! EvalConfigResult {
    () => {
        pub enum EvalConfigResult { True , False { reason : CfgEntry , reason_span : Span } , }
    };
}

EvalConfigResult!()