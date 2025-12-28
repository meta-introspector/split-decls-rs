macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! Sender {
    () => {
        deps!();
        # [doc = " A means of transmitting a single value to another task."] # [doc = ""] # [doc = " This is created by the [`channel`] function."] pub struct Sender < T > { inner : Arc < Inner < T > > , }
    };
}

Sender!();