macro_rules! deps {
    () => {
        IntoIter!();
        Value!();
    };
}

macro_rules! IntoIterImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type IntoIterImpl = indexmap :: map :: IntoIter < String , Value > ;
    };
}

IntoIterImpl!();