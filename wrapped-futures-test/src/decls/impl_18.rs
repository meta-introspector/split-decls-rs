macro_rules! deps {
    () => {
        PanicSpawner!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Spawn for PanicSpawner { fn spawn_obj (& self , _future : FutureObj < 'static , () >) -> Result < () , SpawnError > { panic ! ("should not spawn") } }
    };
}

impl_18!()