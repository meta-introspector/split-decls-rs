macro_rules! deps {
    () => {
        Service!();
        SetServiceResponse!();
        WriteMode!();
        MessageKind!();
        RequestWriter!();
        Transport!();
        Error!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [async_trait (? Send)] impl < T : Transport + ? Sized > Transport for & mut T { async fn handshake < 'a > (& mut self , service : Service , extra_parameters : & 'a [(& 'a str , Option < & 'a str >)] ,) -> Result < SetServiceResponse < '_ > , Error > { self . deref_mut () . handshake (service , extra_parameters) . await } fn request (& mut self , write_mode : WriteMode , on_into_read : MessageKind , trace : bool ,) -> Result < RequestWriter < '_ > , Error > { self . deref_mut () . request (write_mode , on_into_read , trace) } }
    };
}

impl_20!();