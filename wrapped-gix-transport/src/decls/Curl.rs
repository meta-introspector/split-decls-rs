macro_rules! deps {
    () => {
        Error!();
        Request!();
        Options!();
        Response!();
    };
}

macro_rules! Curl {
    () => {
        deps!();
        # [doc = " A utility to abstract interactions with curl handles."] pub struct Curl { req : SyncSender < remote :: Request > , res : Receiver < remote :: Response > , handle : Option < thread :: JoinHandle < Result < () , Error > > > , config : http :: Options , }
    };
}

Curl!();