macro_rules! deps {
    () => {
        IntoValues!();
        Value!();
    };
}

macro_rules! IntoValuesImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type IntoValuesImpl = indexmap :: map :: IntoValues < String , Value > ;
    };
}

IntoValuesImpl!()