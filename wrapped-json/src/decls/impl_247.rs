macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl Default for & Value { fn default () -> Self { const DEFAULT : Value = Value :: Null ; & DEFAULT } }
    };
}

impl_247!();