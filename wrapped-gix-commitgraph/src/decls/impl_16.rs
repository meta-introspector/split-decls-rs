macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Debug for Commit < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Commit {{ id: {}, lex_pos: {}, generation: {}, root_tree_id: {}, parent1: {:?}, parent2: {:?} }}" , self . id () , self . pos , self . generation () , self . root_tree_id () , self . parent1 , self . parent2 ,) } }
    };
}

impl_16!()