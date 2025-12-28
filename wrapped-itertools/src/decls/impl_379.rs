macro_rules! deps {
    () => {
        ArrayBuilder!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl < T , const N : usize > Drop for ArrayBuilder < T , N > { fn drop (& mut self) { unsafe { core :: ptr :: drop_in_place (self . as_mut ()) } } }
    };
}

impl_379!()