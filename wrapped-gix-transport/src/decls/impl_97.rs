macro_rules! deps {
    () => {
        SetServiceResponse!();
        Http!();
        Protocol!();
        RequestWriter!();
        PostResponse!();
        Service!();
        Transport!();
        Error!();
        MessageKind!();
        GetResponse!();
        WriteMode!();
        HeadersThenBody!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < H : Http > blocking_io :: Transport for Transport < H > { fn handshake < 'a > (& mut self , service : Service , extra_parameters : & 'a [(& 'a str , Option < & 'a str >)] ,) -> Result < SetServiceResponse < '_ > , client :: Error > { let url = append_url (self . url . as_ref () , & format ! ("info/refs?service={}" , service . as_str ())) ; let static_headers = [Cow :: Borrowed (self . user_agent_header)] ; let mut dynamic_headers = Vec :: < Cow < '_ , str > > :: new () ; if self . desired_version != Protocol :: V1 || ! extra_parameters . is_empty () { let mut parameters = if self . desired_version != Protocol :: V1 { let mut p = format ! ("version={}" , self . desired_version as usize) ; if ! extra_parameters . is_empty () { p . push (':') ; } p } else { String :: new () } ; parameters . push_str (& extra_parameters . iter () . map (| (key , value) | match value { Some (value) => format ! ("{key}={value}") , None => key . to_string () , }) . collect :: < Vec < _ > > () . join (":") ,) ; dynamic_headers . push (format ! ("Git-Protocol: {parameters}") . into ()) ; } self . add_basic_auth_if_present (& mut dynamic_headers) ? ; let GetResponse { headers , body } = self . http . get (url . as_ref () , & self . url , static_headers . iter () . chain (& dynamic_headers)) ? ; < Transport < H > > :: check_content_type (service , "advertisement" , headers) ? ; let line_reader = self . line_provider . get_or_insert_with (| | StreamingPeekableIter :: new (body , & [PacketLineRef :: Flush] , self . trace)) ; let line_ = line_reader . peek_line () . ok_or (client :: Error :: ExpectedLine ("capabilities, version or service")) ? ? ? ; let line = line_ . as_text () . ok_or (client :: Error :: ExpectedLine ("text")) ? ; if let Some (announced_service) = line . as_bstr () . strip_prefix (b"# service=") { if announced_service != service . as_str () . as_bytes () { return Err (client :: Error :: Http (Error :: Detail { description : format ! ("Expected to see service {:?}, but got {:?}" , service . as_str () , announced_service) , })) ; } line_reader . as_read () . read_to_end (& mut Vec :: new ()) ? ; } let Handshake { capabilities , refs , protocol : actual_protocol , } = Handshake :: from_lines_with_version_detection (line_reader) ? ; self . actual_version = actual_protocol ; self . service = Some (service) ; Ok (SetServiceResponse { actual_protocol , capabilities , refs , }) } fn request (& mut self , write_mode : client :: WriteMode , on_into_read : MessageKind , trace : bool ,) -> Result < RequestWriter < '_ > , client :: Error > { let service = self . service . ok_or (client :: Error :: MissingHandshake) ? ; let url = append_url (& self . url , service . as_str ()) ; let static_headers = & [Cow :: Borrowed (self . user_agent_header) , Cow :: Owned (format ! ("Content-Type: application/x-{}-request" , service . as_str ())) , format ! ("Accept: application/x-{}-result" , service . as_str ()) . into () ,] ; let mut dynamic_headers = Vec :: new () ; self . add_basic_auth_if_present (& mut dynamic_headers) ? ; if self . actual_version != Protocol :: V1 { dynamic_headers . push (Cow :: Owned (format ! ("Git-Protocol: version={}" , self . actual_version as usize))) ; } let PostResponse { headers , body , post_body , } = self . http . post (& url , & self . url , static_headers . iter () . chain (& dynamic_headers) , write_mode . into () ,) ? ; let line_provider = self . line_provider . as_mut () . expect ("handshake to have been called first") ; line_provider . replace (body) ; Ok (RequestWriter :: new_from_bufread (post_body , Box :: new (HeadersThenBody :: < H , _ > { service , headers : Some (headers) , body : line_provider . as_read_without_sidebands () , }) , write_mode , on_into_read , trace ,)) } }
    };
}

impl_97!()