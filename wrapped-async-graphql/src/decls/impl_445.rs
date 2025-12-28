macro_rules! deps {
    () => {
        MetaInputValue!();
        InputValue!();
        Deprecation!();
        TypeRef!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl InputValue { # [doc = " Create a GraphQL input value type"] # [inline] pub fn new (name : impl Into < String > , ty : impl Into < TypeRef >) -> Self { Self { name : name . into () , description : None , ty : ty . into () , default_value : None , inaccessible : false , tags : Vec :: new () , directives : vec ! [] , deprecation : Deprecation :: NoDeprecated , } } impl_set_description ! () ; impl_set_inaccessible ! () ; impl_set_tags ! () ; impl_directive ! () ; impl_set_deprecation ! () ; # [doc = " Set the default value"] # [inline] pub fn default_value (self , value : impl Into < Value >) -> Self { Self { default_value : Some (value . into ()) , .. self } } pub (crate) fn to_meta_input_value (& self) -> MetaInputValue { MetaInputValue { name : self . name . clone () , description : self . description . clone () , ty : self . ty . to_string () , deprecation : self . deprecation . clone () , default_value : self . default_value . as_ref () . map (std :: string :: ToString :: to_string) , visible : None , inaccessible : self . inaccessible , tags : self . tags . clone () , is_secret : false , directive_invocations : to_meta_directive_invocation (self . directives . clone ()) , } } }
    };
}

impl_445!();