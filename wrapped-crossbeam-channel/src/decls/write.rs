macro_rules! deps {
    () => {
        Token!();
        Sender!();
        SenderFlavor!();
    };
}

macro_rules! write {
    () => {
        deps!();
        # [doc = " Writes a message into the channel."] pub (crate) unsafe fn write < T > (s : & Sender < T > , token : & mut Token , msg : T) -> Result < () , T > { unsafe { match & s . flavor { SenderFlavor :: Array (chan) => chan . write (token , msg) , SenderFlavor :: List (chan) => chan . write (token , msg) , SenderFlavor :: Zero (chan) => chan . write (token , msg) , } } }
    };
}

write!();