macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! Receiver {
    () => {
        deps!();
        # [doc = " A future for a value that will be provided by another asynchronous task."] # [doc = ""] # [doc = " This is created by the [`channel`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Receiver < T > { inner : Arc < Inner < T > > , }
    };
}

Receiver!()