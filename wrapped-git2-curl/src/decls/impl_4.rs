macro_rules! deps {
    () => {
        CurlTransport!();
        CurlSubtransport!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl SmartSubtransport for CurlTransport { fn action (& self , url : & str , action : Service ,) -> Result < Box < dyn SmartSubtransportStream > , Error > { let mut base_url = self . base_url . lock () . unwrap () ; if base_url . len () == 0 { * base_url = url . to_string () ; } let (service , path , method) = match action { Service :: UploadPackLs => ("upload-pack" , "/info/refs?service=git-upload-pack" , "GET") , Service :: UploadPack => ("upload-pack" , "/git-upload-pack" , "POST") , Service :: ReceivePackLs => { ("receive-pack" , "/info/refs?service=git-receive-pack" , "GET") } Service :: ReceivePack => ("receive-pack" , "/git-receive-pack" , "POST") , } ; info ! ("action {} {}" , service , path) ; Ok (Box :: new (CurlSubtransport { handle : self . handle . clone () , service : service , url_path : path , base_url : self . base_url . clone () , method : method , reader : None , sent_request : false , })) } fn close (& self) -> Result < () , Error > { Ok (()) } }
    };
}

impl_4!();