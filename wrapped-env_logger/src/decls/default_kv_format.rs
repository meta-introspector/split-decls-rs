macro_rules! deps {
    () => {
        DefaultVisitSource!();
        Formatter!();
    };
}

macro_rules! default_kv_format {
    () => {
        deps!();
        # [doc = " Default Key Value Format"] # [doc = ""] # [doc = " This function is intended to be passed to"] # [doc = " [`Builder::format_key_values`](crate::Builder::format_key_values)."] # [doc = ""] # [doc = " This is the default key/value format. Which uses an \"=\" as the separator between the key and"] # [doc = " value and a \" \" between each pair."] # [doc = ""] # [doc = " For example: `ip=127.0.0.1 port=123456 path=/example`"] pub fn default_kv_format (formatter : & mut Formatter , fields : & dyn Source) -> io :: Result < () > { fields . visit (& mut DefaultVisitSource (formatter)) . map_err (| e | io :: Error :: new (io :: ErrorKind :: Other , e)) }
    };
}

default_kv_format!();