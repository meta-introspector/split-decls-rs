macro_rules! deps {
    () => {
        Result!();
        InputValue!();
        MetaInputValue!();
        MetaType!();
        InputObject!();
        Object!();
        Field!();
        SchemaError!();
        Registry!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        impl InputObject { # [doc = " Create a GraphQL input object type"] # [inline] pub fn new (name : impl Into < String >) -> Self { Self { name : name . into () , description : None , fields : Default :: default () , oneof : false , inaccessible : false , tags : Vec :: new () , directives : Vec :: new () , } } impl_set_description ! () ; impl_set_inaccessible ! () ; impl_set_tags ! () ; impl_directive ! () ; # [doc = " Add a field"] # [inline] pub fn field (mut self , field : InputValue) -> Self { assert ! (! self . fields . contains_key (& field . name) , "Field `{}` already exists" , field . name) ; self . fields . insert (field . name . clone () , field) ; self } # [doc = " Indicates this Input Object is a OneOf Input Object"] pub fn oneof (self) -> Self { Self { oneof : true , .. self } } # [doc = " Returns the type name"] # [inline] pub fn type_name (& self) -> & str { & self . name } pub (crate) fn register (& self , registry : & mut Registry) -> Result < () , super :: SchemaError > { let mut input_fields = IndexMap :: new () ; for field in self . fields . values () { input_fields . insert (field . name . clone () , MetaInputValue { name : field . name . clone () , description : field . description . clone () , ty : field . ty . to_string () , deprecation : field . deprecation . clone () , default_value : field . default_value . as_ref () . map (ToString :: to_string) , visible : None , inaccessible : self . inaccessible , tags : self . tags . clone () , is_secret : false , directive_invocations : to_meta_directive_invocation (field . directives . clone ()) , } ,) ; } registry . types . insert (self . name . clone () , MetaType :: InputObject { name : self . name . clone () , description : self . description . clone () , input_fields , visible : None , inaccessible : self . inaccessible , tags : self . tags . clone () , rust_typename : None , oneof : self . oneof , directive_invocations : to_meta_directive_invocation (self . directives . clone ()) , } ,) ; Ok (()) } }
    };
}

impl_441!()