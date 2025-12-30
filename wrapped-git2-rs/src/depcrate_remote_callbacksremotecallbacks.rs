// Generated macro for RemoteCallbacks (struct)
macro_rules! Depcrate_remote_callbacksRemoteCallbacks {
() => {
// Module: crate::remote_callbacks
// Provides: {"RemoteCallbacks"}
// Dependencies: {}
# [doc = " A structure to contain the callbacks which are invoked when a repository is"] # [doc = " being updated or downloaded."] # [doc = ""] # [doc = " These callbacks are used to manage facilities such as authentication,"] # [doc = " transfer progress, etc."] pub struct RemoteCallbacks < 'a > { push_progress : Option < Box < PushTransferProgress < 'a > > > , progress : Option < Box < IndexerProgress < 'a > > > , pack_progress : Option < Box < PackProgress < 'a > > > , credentials : Option < Box < Credentials < 'a > > > , sideband_progress : Option < Box < TransportMessage < 'a > > > , update_tips : Option < Box < UpdateTips < 'a > > > , certificate_check : Option < Box < CertificateCheck < 'a > > > , push_update_reference : Option < Box < PushUpdateReference < 'a > > > , push_negotiation : Option < Box < PushNegotiation < 'a > > > , }
};
}
