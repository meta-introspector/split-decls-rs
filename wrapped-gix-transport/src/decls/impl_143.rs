macro_rules! deps {
    () => {
        WriteMode!();
        Service!();
        MessageKind!();
        Transport!();
        RequestWriter!();
        SetServiceResponse!();
        Error!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < T : Transport + ? Sized > Transport for & mut T { fn handshake < 'a > (& mut self , service : Service , extra_parameters : & 'a [(& 'a str , Option < & 'a str >)] ,) -> Result < SetServiceResponse < '_ > , Error > { self . deref_mut () . handshake (service , extra_parameters) } fn request (& mut self , write_mode : WriteMode , on_into_read : MessageKind , trace : bool ,) -> Result < RequestWriter < '_ > , Error > { self . deref_mut () . request (write_mode , on_into_read , trace) } }
    };
}

impl_143!()