macro_rules! deps {
    () => {
        Token!();
        ParseAttempt!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < R > ParseAttempt < R > { pub fn get_rule (& self) -> Option < & R > { match self { ParseAttempt :: Rule (r) => Some (r) , ParseAttempt :: Token => None , } } }
    };
}

impl_96!();