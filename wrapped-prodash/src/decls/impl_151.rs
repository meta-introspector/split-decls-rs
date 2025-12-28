macro_rules! deps {
    () => {
        Adjacency!();
        Level!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl IndexMut < Level > for Adjacency { fn index_mut (& mut self , index : Level) -> & mut Self :: Output { self . get_mut (index) . expect ("adjacency index in bound") } }
    };
}

impl_151!()