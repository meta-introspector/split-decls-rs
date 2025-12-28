macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_873 {
    () => {
        deps!();
        impl Spawn for FuturesUnordered < FutureObj < '_ , () > > { fn spawn_obj (& self , future_obj : FutureObj < 'static , () >) -> Result < () , SpawnError > { self . push (future_obj) ; Ok (()) } }
    };
}

impl_873!()