macro_rules! deps {
    () => {
        EntryRef!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl std :: fmt :: Display for EntryRef < '_ , '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:>6o} {:>6} {}\t{}" , self . mode () , self . mode () . as_str () , self . id () . shorten_or_id () , self . filename ()) } }
    };
}

impl_236!()