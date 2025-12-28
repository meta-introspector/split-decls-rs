macro_rules! FromStream {
    () => {
        # [doc = " A concurrent for each implementation from a `Stream`"] # [pin_project :: pin_project] # [derive (Debug)] pub struct FromStream < S : Stream > { # [pin] stream : S , }
    };
}

FromStream!();