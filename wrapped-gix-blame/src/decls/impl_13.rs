macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl SubAssign < u32 > for Offset { fn sub_assign (& mut self , rhs : u32) { match self { Self :: Added (added) => { if rhs > * added { * self = Self :: Deleted (rhs - * added) ; } else { * self = Self :: Added (* added - rhs) ; } } Self :: Deleted (deleted) => * self = Self :: Deleted (* deleted + rhs) , } } }
    };
}

impl_13!()