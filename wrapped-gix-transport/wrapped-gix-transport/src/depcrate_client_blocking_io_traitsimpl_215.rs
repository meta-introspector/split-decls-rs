// Generated macro for impl_215 (impl)
macro_rules! Depcrate_client_blocking_io_traitsimpl_215 {
() => {
// Module: crate::client::blocking_io::traits
// Provides: {"impl_215"}
// Dependencies: {}
impl < T : Transport > TransportV2Ext for T { fn invoke < 'a > (& mut self , command : & str , capabilities : impl Iterator < Item = (& 'a str , Option < impl AsRef < str > >) > + 'a , arguments : Option < impl Iterator < Item = BString > > , trace : bool ,) -> Result < Box < dyn ExtendedBufRead < '_ > + Unpin + '_ > , Error > { let mut writer = self . request (WriteMode :: OneLfTerminatedLinePerWriteCall , MessageKind :: Flush , trace) ? ; writer . write_all (format ! ("command={command}") . as_bytes ()) ? ; for (name , value) in capabilities { match value { Some (value) => writer . write_all (format ! ("{name}={}" , value . as_ref ()) . as_bytes ()) , None => writer . write_all (name . as_bytes ()) , } ? ; } if let Some (arguments) = arguments { writer . write_message (MessageKind :: Delimiter) ? ; for argument in arguments { writer . write_all (argument . as_ref ()) ? ; } } Ok (writer . into_read () ?) } }
};
}
