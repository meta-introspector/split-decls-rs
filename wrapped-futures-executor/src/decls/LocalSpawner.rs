macro_rules! deps {
    () => {
        Incoming!();
        LocalPool!();
    };
}

macro_rules! LocalSpawner {
    () => {
        deps!();
        # [doc = " A handle to a [`LocalPool`] that implements [`Spawn`](futures_task::Spawn)."] # [derive (Clone , Debug)] pub struct LocalSpawner { incoming : Weak < Incoming > , }
    };
}

LocalSpawner!()