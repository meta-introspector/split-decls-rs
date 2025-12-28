macro_rules! deps {
    () => {
        Value!();
        IterMut!();
    };
}

macro_rules! IterMutImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type IterMutImpl < 'a > = indexmap :: map :: IterMut < 'a , String , Value > ;
    };
}

IterMutImpl!()