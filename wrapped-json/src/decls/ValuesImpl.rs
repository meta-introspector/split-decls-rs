macro_rules! deps {
    () => {
        Value!();
        Values!();
    };
}

macro_rules! ValuesImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type ValuesImpl < 'a > = indexmap :: map :: Values < 'a , String , Value > ;
    };
}

ValuesImpl!()