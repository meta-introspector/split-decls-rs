macro_rules! deps {
    () => {
        Value!();
        IntoValues!();
    };
}

macro_rules! IntoValuesImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type IntoValuesImpl = indexmap :: map :: IntoValues < String , Value > ;
    };
}

IntoValuesImpl!();