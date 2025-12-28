macro_rules! AggregateError {
    () => {
        # [doc = " A collection of errors."] # [repr (transparent)] pub struct AggregateError < E > { pub (crate) inner : Vec < E > , }
    };
}

AggregateError!();