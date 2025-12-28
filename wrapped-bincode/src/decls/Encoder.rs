macro_rules! deps {
    () => {
        Config!();
        Sealed!();
        Writer!();
    };
}

macro_rules! Encoder {
    () => {
        deps!();
        # [doc = " Helper trait to encode basic types into."] pub trait Encoder : Sealed { # [doc = " The concrete [Writer] type"] type W : Writer ; # [doc = " The concrete [Config] type"] type C : Config ; # [doc = " Returns a mutable reference to the writer"] fn writer (& mut self) -> & mut Self :: W ; # [doc = " Returns a reference to the config"] fn config (& self) -> & Self :: C ; }
    };
}

Encoder!();