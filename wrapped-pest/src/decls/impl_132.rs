macro_rules! deps {
    () => {
        RuleType!();
        Op!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < R : RuleType > BitOr for Op < R > { type Output = Self ; fn bitor (mut self , rhs : Self) -> Self { fn assign_next < R : RuleType > (op : & mut Op < R > , next : Op < R >) { if let Some (ref mut child) = op . next { assign_next (child , next) ; } else { op . next = Some (Box :: new (next)) ; } } assign_next (& mut self , rhs) ; self } }
    };
}

impl_132!()