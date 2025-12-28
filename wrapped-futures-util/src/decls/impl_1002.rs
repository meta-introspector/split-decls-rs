macro_rules! deps {
    () => {
        Executor01As03!();
        Executor01Future!();
        Send!();
    };
}

macro_rules! impl_1002 {
    () => {
        deps!();
        impl < Ex > Spawn03 for Executor01As03 < Ex > where Ex : Executor01 < Executor01Future > + Clone + Send + 'static , { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError03 > { let future = future . unit_error () . compat () ; self . executor01 . execute (future) . map_err (| _ | SpawnError03 :: shutdown ()) } }
    };
}

impl_1002!()