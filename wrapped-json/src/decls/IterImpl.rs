macro_rules! deps {
    () => {
        Value!();
        Iter!();
    };
}

macro_rules! IterImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type IterImpl < 'a > = indexmap :: map :: Iter < 'a , String , Value > ;
    };
}

IterImpl!()