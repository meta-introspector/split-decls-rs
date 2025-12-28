macro_rules! deps {
    () => {
        Registry!();
        Union!();
        MetaType!();
        Result!();
        SchemaError!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl Union { # [doc = " Create a GraphQL union type"] # [inline] pub fn new (name : impl Into < String >) -> Self { Self { name : name . into () , description : None , possible_types : Default :: default () , inaccessible : false , tags : Vec :: new () , directives : Vec :: new () , } } impl_set_description ! () ; impl_set_inaccessible ! () ; impl_set_tags ! () ; impl_directive ! () ; # [doc = " Add a possible type to the union that must be an object"] # [inline] pub fn possible_type (mut self , ty : impl Into < String >) -> Self { self . possible_types . insert (ty . into ()) ; self } # [doc = " Returns the type name"] # [inline] pub fn type_name (& self) -> & str { & self . name } pub (crate) fn register (& self , registry : & mut Registry) -> Result < () , SchemaError > { registry . types . insert (self . name . clone () , MetaType :: Union { name : self . name . clone () , description : self . description . clone () , possible_types : self . possible_types . clone () , visible : None , inaccessible : self . inaccessible , tags : self . tags . clone () , rust_typename : None , directive_invocations : to_meta_directive_invocation (self . directives . clone ()) , } ,) ; Ok (()) } }
    };
}

impl_509!()