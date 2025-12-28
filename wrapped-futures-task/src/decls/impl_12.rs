macro_rules! deps {
    () => {
        SpawnError!();
        LocalFutureObj!();
        LocalSpawn!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < Sp : ? Sized + LocalSpawn > LocalSpawn for & mut Sp { fn spawn_local_obj (& self , future : LocalFutureObj < 'static , () >) -> Result < () , SpawnError > { Sp :: spawn_local_obj (self , future) } fn status_local (& self) -> Result < () , SpawnError > { Sp :: status_local (self) } }
    };
}

impl_12!();