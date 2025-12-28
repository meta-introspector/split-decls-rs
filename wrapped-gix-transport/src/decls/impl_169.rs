macro_rules! deps {
    () => {
        Connection!();
        Transport!();
        MessageKind!();
        Service!();
        Error!();
        ConnectMode!();
        WriteMode!();
        RequestWriter!();
        SetServiceResponse!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        # [async_trait (? Send)] impl < R , W > client :: async_io :: Transport for Connection < R , W > where R : AsyncRead + Unpin , W : AsyncWrite + Unpin , { async fn handshake < 'a > (& mut self , service : Service , extra_parameters : & 'a [(& 'a str , Option < & 'a str >)] ,) -> Result < SetServiceResponse < '_ > , client :: Error > { if self . state . mode == git :: ConnectMode :: Daemon { let mut line_writer = Writer :: new (& mut self . writer) ; line_writer . enable_binary_mode () ; line_writer . write_all (& git :: message :: connect (service , self . state . desired_version , & self . state . path , self . state . virtual_host . as_ref () , extra_parameters ,)) . await ? ; line_writer . flush () . await ? ; } let Handshake { capabilities , refs , protocol : actual_protocol , } = Handshake :: from_lines_with_version_detection (& mut self . line_provider) . await ? ; Ok (SetServiceResponse { actual_protocol , capabilities , refs , }) } fn request (& mut self , write_mode : client :: WriteMode , on_into_read : client :: MessageKind , trace : bool ,) -> Result < RequestWriter < '_ > , client :: Error > { Ok (RequestWriter :: new_from_bufread (& mut self . writer , Box :: new (self . line_provider . as_read_without_sidebands ()) , write_mode , on_into_read , trace ,)) } }
    };
}

impl_169!();