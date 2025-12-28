macro_rules! deps {
    () => {
        ServerError!();
        Any!();
        InputValueError!();
        ErrorExtensionValues!();
        InputType!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T : InputType > InputValueError < T > { fn new (message : String , extensions : Option < ErrorExtensionValues >) -> Self { Self { message , extensions , phantom : PhantomData , } } # [doc = " The expected input type did not match the actual input type."] # [must_use] pub fn expected_type (actual : Value) -> Self { Self :: new (format ! (r#"Expected input type "{}", found {}."# , T :: type_name () , actual) , None ,) } # [doc = " A custom error message."] # [doc = ""] # [doc = " Any type that implements `Display` is automatically converted to this if"] # [doc = " you use the `?` operator."] # [must_use] pub fn custom (msg : impl Display) -> Self { Self :: new (format ! (r#"Failed to parse "{}": {}"# , T :: type_name () , msg) , None ,) } # [doc = " Propagate the error message to a different type."] pub fn propagate < U : InputType > (self) -> InputValueError < U > { if T :: type_name () != U :: type_name () { InputValueError :: new (format ! (r#"{} (occurred while parsing "{}")"# , self . message , U :: type_name ()) , self . extensions ,) } else { InputValueError :: new (self . message , self . extensions) } } # [doc = " Set an extension value."] pub fn with_extension (mut self , name : impl AsRef < str > , value : impl Into < Value >) -> Self { self . extensions . get_or_insert_with (ErrorExtensionValues :: default) . set (name , value) ; self } # [doc = " Convert the error into a server error."] pub fn into_server_error (self , pos : Pos) -> ServerError { let mut err = ServerError :: new (self . message , Some (pos)) ; err . extensions = self . extensions ; err } }
    };
}

impl_38!();