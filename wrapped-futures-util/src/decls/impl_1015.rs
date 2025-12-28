macro_rules! deps {
    () => {
        Compat01As03!();
    };
}

macro_rules! impl_1015 {
    () => {
        deps!();
        impl < Fut : Future01 > Future03 for Compat01As03 < Fut > { type Output = Result < Fut :: Item , Fut :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> task03 :: Poll < Self :: Output > { poll_01_to_03 (self . in_notify (cx , Future01 :: poll)) } }
    };
}

impl_1015!();