macro_rules! deps {
    () => {
        FileSnapshot!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < T : std :: fmt :: Debug > FileSnapshot < T > { # [doc = " A way for users to create 'fake' snapshot from `value` that isn't actually linked to a file on disk."] # [doc = ""] # [doc = " This is useful if there are alternative ways of obtaining the contained instance as fallback to trying"] # [doc = " to read it from disk."] pub fn new (value : T) -> Self { FileSnapshot { value , modified : std :: time :: UNIX_EPOCH , } } }
    };
}

impl_7!();