macro_rules! deps {
    () => {
        ExtendedBufRead!();
        WriteMode!();
        TransportV2Ext!();
        Transport!();
        Error!();
        MessageKind!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < T : Transport > TransportV2Ext for T { fn invoke < 'a > (& mut self , command : & str , capabilities : impl Iterator < Item = (& 'a str , Option < impl AsRef < str > >) > + 'a , arguments : Option < impl Iterator < Item = BString > > , trace : bool ,) -> Result < Box < dyn ExtendedBufRead < '_ > + Unpin + '_ > , Error > { let mut writer = self . request (WriteMode :: OneLfTerminatedLinePerWriteCall , MessageKind :: Flush , trace) ? ; writer . write_all (format ! ("command={command}") . as_bytes ()) ? ; for (name , value) in capabilities { match value { Some (value) => writer . write_all (format ! ("{name}={}" , value . as_ref ()) . as_bytes ()) , None => writer . write_all (name . as_bytes ()) , } ? ; } if let Some (arguments) = arguments { writer . write_message (MessageKind :: Delimiter) ? ; for argument in arguments { writer . write_all (argument . as_ref ()) ? ; } } Ok (writer . into_read () ?) } }
    };
}

impl_145!();