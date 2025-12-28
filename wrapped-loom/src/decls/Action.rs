macro_rules! Action {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq)] pub (super) enum Action { # [doc = " Read lock"] Read , # [doc = " Write lock"] Write , }
    };
}

Action!()