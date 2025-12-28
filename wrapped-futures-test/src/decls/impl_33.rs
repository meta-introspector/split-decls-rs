macro_rules! deps {
    () => {
        RecordSpawner!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Spawn for RecordSpawner { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { self . spawned . borrow_mut () . push (future) ; Ok (()) } }
    };
}

impl_33!();