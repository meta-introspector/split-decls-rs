macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_874 {
    () => {
        deps!();
        impl LocalSpawn for FuturesUnordered < LocalFutureObj < '_ , () > > { fn spawn_local_obj (& self , future_obj : LocalFutureObj < 'static , () >) -> Result < () , SpawnError > { self . push (future_obj) ; Ok (()) } }
    };
}

impl_874!();