macro_rules! ValueIter {
    () => {
        pub (crate) struct ValueIter < 'll > { cur : Option < & 'll Value > , step : unsafe extern "C" fn (& 'll Value) -> Option < & 'll Value > , }
    };
}

ValueIter!();