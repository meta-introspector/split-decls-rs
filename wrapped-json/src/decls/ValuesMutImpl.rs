macro_rules! deps {
    () => {
        Value!();
        ValuesMut!();
    };
}

macro_rules! ValuesMutImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type ValuesMutImpl < 'a > = indexmap :: map :: ValuesMut < 'a , String , Value > ;
    };
}

ValuesMutImpl!()