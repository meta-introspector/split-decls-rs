macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < R , W > Connection < R , W > { # [doc = " Optionally set the URL to be returned when asked for it if `Some` or calculate a default for `None`."] # [doc = ""] # [doc = " The URL is required as parameter for authentication helpers which are called in transports"] # [doc = " that support authentication. Even though plain git transports don't support that, this"] # [doc = " may well be the case in custom transports."] pub fn custom_url (mut self , url : Option < BString >) -> Self { self . state . custom_url = url ; self } # [doc = " Return the inner reader and writer"] pub fn into_inner (self) -> (R , W) { (self . line_provider . into_inner () , self . writer) } }
    };
}

impl_174!()