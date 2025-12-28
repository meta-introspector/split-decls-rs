macro_rules! deps {
    () => {
        FutureObj!();
        SpawnError!();
        Spawn!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < Sp : ? Sized + Spawn > Spawn for & mut Sp { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { Sp :: spawn_obj (self , future) } fn status (& self) -> Result < () , SpawnError > { Sp :: status (self) } }
    };
}

impl_10!()