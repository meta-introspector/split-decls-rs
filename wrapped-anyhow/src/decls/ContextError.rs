macro_rules! ContextError {
    () => {
        # [repr (C)] pub (crate) struct ContextError < C , E > { pub context : C , pub error : E , }
    };
}

ContextError!();