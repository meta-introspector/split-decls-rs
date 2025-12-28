macro_rules! deps {
    () => {
        Pending!();
        UserBody!();
        Error!();
        Result!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl UserBody { pub (crate) fn new () -> UserBody { UserBody { data_func : data_noop , userdata : std :: ptr :: null_mut () , } } pub (crate) fn poll_data (& mut self , cx : & mut Context < '_ > ,) -> Poll < Option < crate :: Result < Frame < Bytes > > > > { let mut out = std :: ptr :: null_mut () ; match (self . data_func) (self . userdata , hyper_context :: wrap (cx) , & mut out) { super :: task :: HYPER_POLL_READY => { if out . is_null () { Poll :: Ready (None) } else { let buf = unsafe { Box :: from_raw (out) } ; Poll :: Ready (Some (Ok (Frame :: data (buf . 0)))) } } super :: task :: HYPER_POLL_PENDING => Poll :: Pending , super :: task :: HYPER_POLL_ERROR => { Poll :: Ready (Some (Err (crate :: Error :: new_body_write_aborted ()))) } unexpected => Poll :: Ready (Some (Err (crate :: Error :: new_body_write (format ! ("unexpected hyper_body_data_func return code {}" , unexpected))))) , } } }
    };
}

impl_247!();