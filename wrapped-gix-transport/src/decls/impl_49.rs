macro_rules! deps {
    () => {
        Handler!();
        Error!();
        StreamOrBuffer!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl curl :: easy :: Handler for Handler { fn write (& mut self , data : & [u8]) -> Result < usize , curl :: easy :: WriteError > { drop (self . send_header . take ()) ; match self . send_data . as_mut () { Some (writer) => writer . write_all (data) . map (| _ | data . len ()) . or (Ok (0)) , None => Ok (0) , } } fn read (& mut self , data : & mut [u8]) -> Result < usize , curl :: easy :: ReadError > { match self . receive_body . as_mut () { Some (StreamOrBuffer :: Stream (reader)) => reader . read (data) . map_err (| _err | curl :: easy :: ReadError :: Abort) , Some (StreamOrBuffer :: Buffer (cursor)) => cursor . read (data) . map_err (| _err | curl :: easy :: ReadError :: Abort) , None => Ok (0) , } } fn header (& mut self , data : & [u8]) -> bool { if let Some (writer) = self . send_header . as_mut () { if self . checked_status { writer . write_all (data) . ok () ; } else { self . checked_status = true ; self . last_status = 200 ; if let Some ((status , err)) = Handler :: parse_status (data , self . follow) { self . last_status = status ; writer . channel . send (Err (io :: Error :: new (if status == 401 { io :: ErrorKind :: PermissionDenied } else if (500 .. 600) . contains (& status) { io :: ErrorKind :: ConnectionAborted } else { io :: ErrorKind :: Other } , err ,))) . ok () ; } } } true } }
    };
}

impl_49!();