macro_rules! deps {
    () => {
        ReceiverFlavor!();
        Receiver!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T > Drop for Receiver < T > { fn drop (& mut self) { unsafe { match & self . flavor { ReceiverFlavor :: Array (chan) => chan . release (| c | c . disconnect ()) , ReceiverFlavor :: List (chan) => chan . release (| c | c . disconnect_receivers ()) , ReceiverFlavor :: Zero (chan) => chan . release (| c | c . disconnect ()) , ReceiverFlavor :: At (_) => { } ReceiverFlavor :: Tick (_) => { } ReceiverFlavor :: Never (_) => { } } } } }
    };
}

impl_24!()