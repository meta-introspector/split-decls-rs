macro_rules! deps {
    () => {
        NoopSpawner!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Spawn for NoopSpawner { fn spawn_obj (& self , _future : FutureObj < 'static , () >) -> Result < () , SpawnError > { Ok (()) } }
    };
}

impl_12!()