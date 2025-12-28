macro_rules! deps {
    () => {
        OwnedOrStaticAtomicBool!();
    };
}

macro_rules! parallel_iter_drop {
    () => {
        deps!();
        # [cfg (feature = "parallel")] # [allow (clippy :: type_complexity)] pub fn parallel_iter_drop < T , U , V > (mut rx_and_join : Option < (std :: sync :: mpsc :: Receiver < T > , std :: thread :: JoinHandle < U > , Option < std :: thread :: JoinHandle < V > > ,) > , should_interrupt : & OwnedOrStaticAtomicBool ,) { let Some ((rx , handle , maybe_handle)) = rx_and_join . take () else { return ; } ; let prev = should_interrupt . swap (true , std :: sync :: atomic :: Ordering :: Relaxed) ; let undo = match & should_interrupt { OwnedOrStaticAtomicBool :: Shared (flag) => * flag , OwnedOrStaticAtomicBool :: Owned { flag , private : false } => flag . as_ref () , OwnedOrStaticAtomicBool :: Owned { private : true , .. } => { drop ((rx , handle , maybe_handle)) ; return ; } } ; drop ((maybe_handle , handle)) ; undo . fetch_update (std :: sync :: atomic :: Ordering :: SeqCst , std :: sync :: atomic :: Ordering :: SeqCst , | current | current . then_some (prev) ,) . ok () ; }
    };
}

parallel_iter_drop!();