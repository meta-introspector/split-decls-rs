macro_rules! deps {
    () => {
        Clone!();
    };
}

macro_rules! Note {
    () => {
        deps!();
        # [doc = " A note attached to a key."] # [derive (Debug , Copy , Clone)] pub enum Note { # [doc = " A piece of information related to a key to help the user."] Informative (& 'static str) , # [doc = " This key works differently than is described by git, explaining the deviation further."] Deviation (& 'static str) , }
    };
}

Note!()