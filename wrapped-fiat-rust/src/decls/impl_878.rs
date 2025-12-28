macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_878 {
    () => {
        deps!();
        impl < 'a , 'b , T , const N : usize > IndexConst < & 'a mut & 'b mut [T ; N] > { # [inline (always)] # [allow (unused)] const fn index_mut (self , i : usize) -> & 'a mut T { & mut self . 0 [i] } }
    };
}

impl_878!()