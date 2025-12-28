macro_rules! deps {
    () => {
        Response!();
        Options!();
        Error!();
        Request!();
    };
}

macro_rules! Remote {
    () => {
        deps!();
        # [doc = " An implementation for HTTP requests via `reqwest`."] pub struct Remote { # [doc = " A worker thread which performs the actual request."] handle : Option < std :: thread :: JoinHandle < Result < () , remote :: Error > > > , # [doc = " A channel to send requests (work) to the worker thread."] request : std :: sync :: mpsc :: SyncSender < remote :: Request > , # [doc = " A channel to receive the result of the prior request."] response : std :: sync :: mpsc :: Receiver < remote :: Response > , # [doc = " A mechanism for configuring the remote."] config : crate :: client :: blocking_io :: http :: Options , }
    };
}

Remote!()