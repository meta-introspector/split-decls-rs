macro_rules! deps {
    () => {
        SenderFlavor!();
        Sender!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T > Drop for Sender < T > { fn drop (& mut self) { unsafe { match & self . flavor { SenderFlavor :: Array (chan) => chan . release (| c | c . disconnect ()) , SenderFlavor :: List (chan) => chan . release (| c | c . disconnect_senders ()) , SenderFlavor :: Zero (chan) => chan . release (| c | c . disconnect ()) , } } } }
    };
}

impl_14!();