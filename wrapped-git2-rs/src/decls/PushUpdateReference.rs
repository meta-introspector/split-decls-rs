macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! PushUpdateReference {
    () => {
        deps!();
        # [doc = " Callback for each updated reference on push."] # [doc = ""] # [doc = " The first argument here is the `refname` of the reference, and the second is"] # [doc = " the status message sent by a server. If the status is `Some` then the update"] # [doc = " was rejected by the remote server with a reason why."] pub type PushUpdateReference < 'a > = dyn FnMut (& str , Option < & str >) -> Result < () , Error > + 'a ;
    };
}

PushUpdateReference!();