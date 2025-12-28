macro_rules! deps {
    () => {
        Service!();
        Error!();
        SmartSubtransportStream!();
    };
}

macro_rules! SmartSubtransport {
    () => {
        deps!();
        # [doc = " Interface used by smart transports."] # [doc = ""] # [doc = " The full-fledged definition of transports has to deal with lots of"] # [doc = " nitty-gritty details of the git protocol, but \"smart transports\" largely"] # [doc = " only need to deal with read() and write() of data over a channel."] # [doc = ""] # [doc = " A smart subtransport is contained within an instance of a smart transport"] # [doc = " and is delegated to in order to actually conduct network activity to push or"] # [doc = " pull data from a remote."] pub trait SmartSubtransport : Send + 'static { # [doc = " Indicates that this subtransport will be performing the specified action"] # [doc = " on the specified URL."] # [doc = ""] # [doc = " This function is responsible for making any network connections and"] # [doc = " returns a stream which can be read and written from in order to"] # [doc = " negotiate the git protocol."] fn action (& self , url : & str , action : Service) -> Result < Box < dyn SmartSubtransportStream > , Error > ; # [doc = " Terminates a connection with the remote."] # [doc = ""] # [doc = " Each subtransport is guaranteed a call to close() between calls to"] # [doc = " action(), except for the following two natural progressions of actions"] # [doc = " against a constant URL."] # [doc = ""] # [doc = " 1. UploadPackLs -> UploadPack"] # [doc = " 2. ReceivePackLs -> ReceivePack"] fn close (& self) -> Result < () , Error > ; }
    };
}

SmartSubtransport!();