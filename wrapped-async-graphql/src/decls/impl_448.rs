macro_rules! deps {
    () => {
        InputValue!();
        TypeRef!();
        Deprecation!();
        InterfaceField!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        impl InterfaceField { # [doc = " Create a GraphQL interface field type"] pub fn new (name : impl Into < String > , ty : impl Into < TypeRef >) -> Self { Self { name : name . into () , description : None , arguments : Default :: default () , ty : ty . into () , deprecation : Deprecation :: NoDeprecated , external : false , requires : None , provides : None , shareable : false , inaccessible : false , tags : Vec :: new () , override_from : None , directives : Vec :: new () , requires_scopes : Vec :: new () , } } impl_set_description ! () ; impl_set_deprecation ! () ; impl_set_external ! () ; impl_set_requires ! () ; impl_set_provides ! () ; impl_set_shareable ! () ; impl_set_inaccessible ! () ; impl_set_tags ! () ; impl_set_override_from ! () ; impl_directive ! () ; # [doc = " Add an argument to the field"] # [inline] pub fn argument (mut self , input_value : InputValue) -> Self { self . arguments . insert (input_value . name . clone () , input_value) ; self } }
    };
}

impl_448!();