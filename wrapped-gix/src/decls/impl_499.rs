macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        # [doc = " Generic modification"] impl Options { # [doc = " An adapter to allow calling any builder method on this instance despite only having a mutable reference."] pub fn modify (& mut self , f : impl FnOnce (Self) -> Self) { * self = f (std :: mem :: take (self)) ; } }
    };
}

impl_499!()