macro_rules! deps {
    () => {
        SeqHandle!();
        Sequence!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl SeqHandle { # [doc = " Tell the Sequence that this expectation has been fully satisfied"] pub fn satisfy (& self) { self . inner . satisfy (self . seq) ; } # [doc = " Verify that this handle was called in the correct order"] pub fn verify < F : Fn () -> String > (& self , desc : F) { self . inner . verify (self . seq , desc) ; } }
    };
}

impl_31!()