macro_rules! deps {
    () => {
        Keys!();
        Value!();
    };
}

macro_rules! KeysImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type KeysImpl < 'a > = indexmap :: map :: Keys < 'a , String , Value > ;
    };
}

KeysImpl!()