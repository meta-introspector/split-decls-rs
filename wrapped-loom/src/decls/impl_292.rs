macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < T > Receiver < T > { # [doc = " Attempts to wait for a value on this receiver, returning an error if the"] # [doc = " corresponding channel has hung up."] # [track_caller] pub fn recv (& self) -> Result < T , std :: sync :: mpsc :: RecvError > { self . object . recv (location ! ()) ; self . receiver . recv () } # [doc = " Attempts to wait for a value on this receiver, returning an error if the"] # [doc = " corresponding channel has hung up, or if it waits more than `timeout`."] pub fn recv_timeout (& self , _timeout : std :: time :: Duration ,) -> Result < T , std :: sync :: mpsc :: RecvTimeoutError > { unimplemented ! ("std::sync::mpsc::Receiver::recv_timeout is not supported yet in Loom.") } # [doc = " Attempts to return a pending value on this receiver without blocking."] pub fn try_recv (& self) -> Result < T , std :: sync :: mpsc :: TryRecvError > { if self . object . is_empty () { return Err (std :: sync :: mpsc :: TryRecvError :: Empty) ; } else { self . recv () . map_err (| e | e . into ()) } } }
    };
}

impl_292!()