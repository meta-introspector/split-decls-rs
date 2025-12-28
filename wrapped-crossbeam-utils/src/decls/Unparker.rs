macro_rules! deps {
    () => {
        Parker!();
        Inner!();
    };
}

macro_rules! Unparker {
    () => {
        deps!();
        # [doc = " Unparks a thread parked by the associated [`Parker`]."] pub struct Unparker { inner : Arc < Inner > , }
    };
}

Unparker!();