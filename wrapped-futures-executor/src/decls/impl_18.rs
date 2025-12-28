macro_rules! deps {
    () => {
        LocalSpawner!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Spawn for LocalSpawner { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { if let Some (incoming) = self . incoming . upgrade () { incoming . borrow_mut () . push (future . into ()) ; Ok (()) } else { Err (SpawnError :: shutdown ()) } } fn status (& self) -> Result < () , SpawnError > { if self . incoming . upgrade () . is_some () { Ok (()) } else { Err (SpawnError :: shutdown ()) } } }
    };
}

impl_18!()