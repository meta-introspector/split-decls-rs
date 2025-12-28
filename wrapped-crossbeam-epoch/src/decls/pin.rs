macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! pin {
    () => {
        deps!();
        # [doc = " Pins the current thread."] # [inline] pub fn pin () -> Guard { with_handle (| handle | handle . pin ()) }
    };
}

pin!()