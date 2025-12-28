macro_rules! deps {
    () => {
        LocalFutureObj!();
        LocalSpawn!();
        SpawnError!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < Sp : ? Sized + LocalSpawn > LocalSpawn for & Sp { fn spawn_local_obj (& self , future : LocalFutureObj < 'static , () >) -> Result < () , SpawnError > { Sp :: spawn_local_obj (self , future) } fn status_local (& self) -> Result < () , SpawnError > { Sp :: status_local (self) } }
    };
}

impl_11!()