macro_rules! deps {
    () => {
        WorkerThread!();
    };
}

macro_rules! BroadcastContext {
    () => {
        deps!();
        # [doc = " Provides context to a closure called by `broadcast`."] pub struct BroadcastContext < 'a > { worker : & 'a WorkerThread , # [doc = " Make sure to prevent auto-traits like `Send` and `Sync`."] _marker : PhantomData < & 'a mut dyn Fn () > , }
    };
}

BroadcastContext!()