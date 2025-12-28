macro_rules! deps {
    () => {
        StreamGroup!();
    };
}

macro_rules! Keyed {
    () => {
        deps!();
        # [doc = " Iterate over items in the stream group with their associated keys."] # [derive (Debug)] # [pin_project :: pin_project] pub struct Keyed < S : Stream > { # [pin] group : StreamGroup < S > , }
    };
}

Keyed!();