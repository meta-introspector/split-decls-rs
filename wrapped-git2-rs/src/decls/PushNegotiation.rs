macro_rules! deps {
    () => {
        PushUpdate!();
        Error!();
    };
}

macro_rules! PushNegotiation {
    () => {
        deps!();
        # [doc = " The callback is called once between the negotiation step and the upload."] # [doc = ""] # [doc = " The argument is a slice containing the updates which will be sent as"] # [doc = " commands to the destination."] # [doc = ""] # [doc = " The push is cancelled if an error is returned."] pub type PushNegotiation < 'a > = dyn FnMut (& [PushUpdate < '_ >]) -> Result < () , Error > + 'a ;
    };
}

PushNegotiation!()