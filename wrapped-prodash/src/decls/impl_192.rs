macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl std :: hash :: Hash for Value { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { let Self { step , done_at , unit , state : our_state , } = self ; done_at . hash (state) ; unit . hash (state) ; our_state . hash (state) ; step . load (Ordering :: Relaxed) . hash (state) ; } }
    };
}

impl_192!()