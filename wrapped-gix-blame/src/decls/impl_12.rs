macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl AddAssign < u32 > for Offset { fn add_assign (& mut self , rhs : u32) { match self { Self :: Added (added) => * self = Self :: Added (* added + rhs) , Self :: Deleted (deleted) => { if rhs > * deleted { * self = Self :: Added (rhs - * deleted) ; } else { * self = Self :: Deleted (* deleted - rhs) ; } } } } }
    };
}

impl_12!();