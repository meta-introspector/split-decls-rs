macro_rules! deps {
    () => {
        Safety!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl Safety { pub fn prefix_str (self) -> & 'static str { match self { Self :: Unsafe => "unsafe " , Self :: Safe => "" , } } # [inline] pub fn is_unsafe (self) -> bool { ! self . is_safe () } # [inline] pub fn is_safe (self) -> bool { match self { Self :: Unsafe => false , Self :: Safe => true , } } }
    };
}

impl_314!();