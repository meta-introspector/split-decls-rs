macro_rules! deps {
    () => {
        Hole!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl < T > Drop for Hole < '_ , T > { # [inline] fn drop (& mut self) { unsafe { let pos = self . pos ; ptr :: write (self . data . get_unchecked_mut (pos) , ptr :: read (& * self . elt)) ; } } }
    };
}

impl_380!();