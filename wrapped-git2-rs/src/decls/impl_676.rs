macro_rules! deps {
    () => {
        Credentials!();
        Progress!();
        CertificateCheckStatus!();
        Cred!();
        CertificateCheck!();
        Cert!();
        IndexerProgress!();
        RemoteCallbacks!();
        TransportMessage!();
        PushUpdateReference!();
        PackBuilderStage!();
        PackProgress!();
        Error!();
        UpdateTips!();
        PushNegotiation!();
        PushUpdate!();
        PushTransferProgress!();
        Oid!();
    };
}

macro_rules! impl_676 {
    () => {
        deps!();
        impl < 'a > RemoteCallbacks < 'a > { # [doc = " Creates a new set of empty callbacks"] pub fn new () -> RemoteCallbacks < 'a > { RemoteCallbacks { credentials : None , progress : None , pack_progress : None , sideband_progress : None , update_tips : None , certificate_check : None , push_update_reference : None , push_progress : None , push_negotiation : None , } } # [doc = " The callback through which to fetch credentials if required."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Prepare a callback to authenticate using the `$HOME/.ssh/id_rsa` SSH key, and"] # [doc = " extracting the username from the URL (i.e. git@github.com:rust-lang/git2-rs.git):"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use git2::{Cred, RemoteCallbacks};"] # [doc = " use std::env;"] # [doc = ""] # [doc = " let mut callbacks = RemoteCallbacks::new();"] # [doc = " callbacks.credentials(|_url, username_from_url, _allowed_types| {"] # [doc = "   Cred::ssh_key("] # [doc = "     username_from_url.unwrap(),"] # [doc = "     None,"] # [doc = "     std::path::Path::new(&format!(\"{}/.ssh/id_rsa\", env::var(\"HOME\").unwrap())),"] # [doc = "     None,"] # [doc = "   )"] # [doc = " });"] # [doc = " ```"] pub fn credentials < F > (& mut self , cb : F) -> & mut RemoteCallbacks < 'a > where F : FnMut (& str , Option < & str > , CredentialType) -> Result < Cred , Error > + 'a , { self . credentials = Some (Box :: new (cb) as Box < Credentials < 'a > >) ; self } # [doc = " The callback through which progress is monitored."] pub fn transfer_progress < F > (& mut self , cb : F) -> & mut RemoteCallbacks < 'a > where F : FnMut (Progress < '_ >) -> bool + 'a , { self . progress = Some (Box :: new (cb) as Box < IndexerProgress < 'a > >) ; self } # [doc = " Textual progress from the remote."] # [doc = ""] # [doc = " Text sent over the progress side-band will be passed to this function"] # [doc = " (this is the 'counting objects' output)."] pub fn sideband_progress < F > (& mut self , cb : F) -> & mut RemoteCallbacks < 'a > where F : FnMut (& [u8]) -> bool + 'a , { self . sideband_progress = Some (Box :: new (cb) as Box < TransportMessage < 'a > >) ; self } # [doc = " Each time a reference is updated locally, the callback will be called"] # [doc = " with information about it."] pub fn update_tips < F > (& mut self , cb : F) -> & mut RemoteCallbacks < 'a > where F : FnMut (& str , Oid , Oid) -> bool + 'a , { self . update_tips = Some (Box :: new (cb) as Box < UpdateTips < 'a > >) ; self } # [doc = " If certificate verification fails, then this callback will be invoked to"] # [doc = " let the caller make the final decision of whether to allow the"] # [doc = " connection to proceed."] pub fn certificate_check < F > (& mut self , cb : F) -> & mut RemoteCallbacks < 'a > where F : FnMut (& Cert < '_ > , & str) -> Result < CertificateCheckStatus , Error > + 'a , { self . certificate_check = Some (Box :: new (cb) as Box < CertificateCheck < 'a > >) ; self } # [doc = " Set a callback to get invoked for each updated reference on a push."] # [doc = ""] # [doc = " The first argument to the callback is the name of the reference and the"] # [doc = " second is a status message sent by the server. If the status is `Some`"] # [doc = " then the push was rejected."] pub fn push_update_reference < F > (& mut self , cb : F) -> & mut RemoteCallbacks < 'a > where F : FnMut (& str , Option < & str >) -> Result < () , Error > + 'a , { self . push_update_reference = Some (Box :: new (cb) as Box < PushUpdateReference < 'a > >) ; self } # [doc = " The callback through which progress of push transfer is monitored"] # [doc = ""] # [doc = " Parameters:"] # [doc = " * current"] # [doc = " * total"] # [doc = " * bytes"] pub fn push_transfer_progress < F > (& mut self , cb : F) -> & mut RemoteCallbacks < 'a > where F : FnMut (usize , usize , usize) + 'a , { self . push_progress = Some (Box :: new (cb) as Box < PushTransferProgress < 'a > >) ; self } # [doc = " Function to call with progress information during pack building."] # [doc = ""] # [doc = " Be aware that this is called inline with pack building operations,"] # [doc = " so performance may be affected."] # [doc = ""] # [doc = " Parameters:"] # [doc = " * stage"] # [doc = " * current"] # [doc = " * total"] pub fn pack_progress < F > (& mut self , cb : F) -> & mut RemoteCallbacks < 'a > where F : FnMut (PackBuilderStage , usize , usize) + 'a , { self . pack_progress = Some (Box :: new (cb) as Box < PackProgress < 'a > >) ; self } # [doc = " The callback is called once between the negotiation step and the upload."] # [doc = ""] # [doc = " The argument to the callback is a slice containing the updates which"] # [doc = " will be sent as commands to the destination."] # [doc = ""] # [doc = " The push is cancelled if the callback returns an error."] pub fn push_negotiation < F > (& mut self , cb : F) -> & mut RemoteCallbacks < 'a > where F : FnMut (& [PushUpdate < '_ >]) -> Result < () , Error > + 'a , { self . push_negotiation = Some (Box :: new (cb) as Box < PushNegotiation < 'a > >) ; self } }
    };
}

impl_676!();