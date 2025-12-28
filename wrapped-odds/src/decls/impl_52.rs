macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [doc = " `RevSlice` compares by logical element sequence."] impl < T , U > PartialEq < [U] > for RevSlice < T > where T : PartialEq < U > , { fn eq (& self , rhs : & [U]) -> bool { if self . len () != rhs . len () { return false ; } for (x , y) in self . into_iter () . zip (rhs) { if x != y { return false ; } } true } }
    };
}

impl_52!()