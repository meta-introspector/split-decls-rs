macro_rules! deps {
    () => {
        Unit!();
        What!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl What { fn values (& self) -> bool { matches ! (self , What :: Values | What :: ValuesAndUnit) } fn unit (& self) -> bool { matches ! (self , What :: Unit | What :: ValuesAndUnit) } }
    };
}

impl_122!();