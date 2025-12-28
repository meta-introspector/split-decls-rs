macro_rules! deps {
    () => {
        Pagerank!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Display for Pagerank { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { write ! (f , "Pagerank Struct:\n\
             InLinks: {:?}\n\
             NumberOutLinks: {:?}\n\
             CurrentAvailableIndex: {}\n\
             KeyToIndex: {:?}\n\
             IndexToKey: {:?}\n\
             Capacity: {}" , self . in_links , self . number_out_links , self . current_available_index , self . key_to_index , self . index_to_key , self . capacity) } }
    };
}

impl_5!()