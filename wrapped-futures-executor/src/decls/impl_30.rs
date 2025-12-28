macro_rules! deps {
    () => {
        ThreadPool!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Spawn for ThreadPool { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { self . spawn_obj_ok (future) ; Ok (()) } }
    };
}

impl_30!()