macro_rules! deps {
    () => {
        SiblingLocation!();
        Level!();
        Adjacency!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl Index < Level > for Adjacency { type Output = SiblingLocation ; fn index (& self , index : Level) -> & Self :: Output { self . get (index) . expect ("adjacency index in bound") } }
    };
}

impl_150!();