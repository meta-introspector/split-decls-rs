macro_rules! macro_144 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::chain()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Chain < S , U > { # [pin] first : Fuse < S >, # [pin] second : Fuse < U >, } }
    };
}

macro_144!();