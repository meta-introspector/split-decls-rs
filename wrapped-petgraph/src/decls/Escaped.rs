macro_rules! Escaped {
    () => {
        # [doc = " Pass Display formatting through a simple escaping filter"] struct Escaped < T > (T) ;
    };
}

Escaped!()