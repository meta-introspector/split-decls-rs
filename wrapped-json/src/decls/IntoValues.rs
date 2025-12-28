macro_rules! deps {
    () => {
        IntoValuesImpl!();
    };
}

macro_rules! IntoValues {
    () => {
        deps!();
        # [doc = " An owning iterator over a serde_json::Map's values."] # [derive (Debug)] pub struct IntoValues { iter : IntoValuesImpl , }
    };
}

IntoValues!();