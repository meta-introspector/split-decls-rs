macro_rules! deps {
    () => {
        SchemaError!();
        MetaType!();
        Registry!();
        Result!();
        Scalar!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl Scalar { # [doc = " Create a GraphQL scalar type"] # [inline] pub fn new (name : impl Into < String >) -> Self { Self { name : name . into () , description : None , specified_by_url : None , validator : None , inaccessible : false , tags : Vec :: new () , directives : Vec :: new () , requires_scopes : Vec :: new () , } } impl_set_description ! () ; impl_set_inaccessible ! () ; impl_set_tags ! () ; impl_directive ! () ; # [doc = " Set the validator"] # [inline] pub fn validator (self , validator : impl Fn (& Value) -> bool + Send + Sync + 'static) -> Self { Self { validator : Some (Arc :: new (validator)) , .. self } } # [inline] pub (crate) fn validate (& self , value : & Value) -> bool { match & self . validator { Some (validator) => (validator) (value) , None => true , } } # [doc = " Set the specified by url"] # [inline] pub fn specified_by_url (self , specified_by_url : impl Into < String >) -> Self { Self { specified_by_url : Some (specified_by_url . into ()) , .. self } } # [doc = " Returns the type name"] # [inline] pub fn type_name (& self) -> & str { & self . name } pub (crate) fn register (& self , registry : & mut Registry) -> Result < () , SchemaError > { registry . types . insert (self . name . clone () , MetaType :: Scalar { name : self . name . clone () , description : self . description . clone () , is_valid : self . validator . clone () , visible : None , inaccessible : self . inaccessible , tags : self . tags . clone () , specified_by_url : self . specified_by_url . clone () , directive_invocations : to_meta_directive_invocation (self . directives . clone ()) , requires_scopes : self . requires_scopes . clone () , } ,) ; Ok (()) } }
    };
}

impl_478!();