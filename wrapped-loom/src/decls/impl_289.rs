macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl < T > Sender < T > { # [doc = " Attempts to send a value on this channel, returning it back if it could"] # [doc = " not be sent."] # [track_caller] pub fn send (& self , msg : T) -> Result < () , std :: sync :: mpsc :: SendError < T > > { self . object . send (location ! ()) ; self . sender . send (msg) } }
    };
}

impl_289!()