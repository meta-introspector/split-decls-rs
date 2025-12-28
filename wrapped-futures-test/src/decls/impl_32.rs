macro_rules! deps {
    () => {
        RecordSpawner!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl RecordSpawner { # [doc = " Create a new instance"] pub fn new () -> Self { Default :: default () } # [doc = " Inspect any futures that were spawned onto this [`Spawn`]."] pub fn spawned (& self) -> Ref < '_ , Vec < FutureObj < 'static , () > > > { self . spawned . borrow () } }
    };
}

impl_32!()