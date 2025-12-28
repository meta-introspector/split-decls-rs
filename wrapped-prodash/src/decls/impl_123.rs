macro_rules! deps {
    () => {
        UnitDisplay!();
        Unit!();
        What!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl UnitDisplay < '_ > { # [doc = " Display everything, values and the unit."] pub fn all (& mut self) -> & Self { self . display = What :: ValuesAndUnit ; self } # [doc = " Display only values."] pub fn values (& mut self) -> & Self { self . display = What :: Values ; self } # [doc = " Display only units."] pub fn unit (& mut self) -> & Self { self . display = What :: Unit ; self } }
    };
}

impl_123!()