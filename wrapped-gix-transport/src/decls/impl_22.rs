macro_rules! deps {
    () => {
        ExtendedBufRead!();
        TransportV2Ext!();
        Error!();
        MessageKind!();
        Transport!();
        WriteMode!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [async_trait (? Send)] impl < T : Transport > TransportV2Ext for T { async fn invoke < 'a > (& mut self , command : & str , capabilities : impl Iterator < Item = (& 'a str , Option < impl AsRef < str > >) > + 'a , arguments : Option < impl Iterator < Item = BString > + 'a > , trace : bool ,) -> Result < Box < dyn ExtendedBufRead < '_ > + Unpin + '_ > , Error > { let mut writer = self . request (WriteMode :: OneLfTerminatedLinePerWriteCall , MessageKind :: Flush , trace) ? ; writer . write_all (format ! ("command={command}") . as_bytes ()) . await ? ; for (name , value) in capabilities { match value { Some (value) => { writer . write_all (format ! ("{}={}" , name , value . as_ref ()) . as_bytes ()) . await } None => writer . write_all (name . as_bytes ()) . await , } ? ; } if let Some (arguments) = arguments { writer . write_message (MessageKind :: Delimiter) . await ? ; for argument in arguments { writer . write_all (argument . as_ref ()) . await ? ; } } Ok (writer . into_read () . await ?) } }
    };
}

impl_22!();