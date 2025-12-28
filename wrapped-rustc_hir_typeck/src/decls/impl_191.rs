macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl Ord for Error < '_ > { fn cmp (& self , other : & Self) -> Ordering { let key = | error : & Error < '_ > | -> usize { match error { Error :: Invalid (..) => 0 , Error :: Extra (_) => 1 , Error :: Missing (_) => 2 , Error :: Swap (..) => 3 , Error :: Permutation (..) => 4 , } } ; match (self , other) { (Error :: Invalid (a , _ , _) , Error :: Invalid (b , _ , _)) => a . cmp (b) , (Error :: Extra (a) , Error :: Extra (b)) => a . cmp (b) , (Error :: Missing (a) , Error :: Missing (b)) => a . cmp (b) , (Error :: Swap (a , b , ..) , Error :: Swap (c , d , ..)) => a . cmp (c) . then (b . cmp (d)) , (Error :: Permutation (a) , Error :: Permutation (b)) => a . cmp (b) , _ => key (self) . cmp (& key (other)) , } } }
    };
}

impl_191!();