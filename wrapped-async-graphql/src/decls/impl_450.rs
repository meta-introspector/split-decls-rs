macro_rules! deps {
    () => {
        Interface!();
        Result!();
        SchemaError!();
        MetaType!();
        Registry!();
        Field!();
        MetaField!();
        InterfaceField!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        impl Interface { # [doc = " Create a GraphQL interface type"] # [inline] pub fn new (name : impl Into < String >) -> Self { Self { name : name . into () , description : None , fields : Default :: default () , implements : Default :: default () , keys : Vec :: new () , extends : false , inaccessible : false , tags : Vec :: new () , directives : Vec :: new () , requires_scopes : Vec :: new () , } } impl_set_description ! () ; impl_set_extends ! () ; impl_set_inaccessible ! () ; impl_set_tags ! () ; impl_directive ! () ; # [doc = " Add a field to the interface type"] # [inline] pub fn field (mut self , field : InterfaceField) -> Self { assert ! (! self . fields . contains_key (& field . name) , "Field `{}` already exists" , field . name) ; self . fields . insert (field . name . clone () , field) ; self } # [doc = " Add an implement to the interface type"] # [inline] pub fn implement (mut self , interface : impl Into < String >) -> Self { let interface = interface . into () ; assert ! (! self . implements . contains (& interface) , "Implement `{}` already exists" , interface) ; self . implements . insert (interface) ; self } # [doc = " Add an entity key"] # [doc = ""] # [doc = " See also: [`Object::key`](crate::dynamic::Object::key)"] pub fn key (mut self , fields : impl Into < String >) -> Self { self . keys . push (fields . into ()) ; self } # [doc = " Returns the type name"] # [inline] pub fn type_name (& self) -> & str { & self . name } # [inline] pub (crate) fn is_entity (& self) -> bool { ! self . keys . is_empty () } pub (crate) fn register (& self , registry : & mut Registry) -> Result < () , SchemaError > { let mut fields = IndexMap :: new () ; for field in self . fields . values () { let mut args = IndexMap :: new () ; for argument in field . arguments . values () { args . insert (argument . name . clone () , argument . to_meta_input_value ()) ; } fields . insert (field . name . clone () , MetaField { name : field . name . clone () , description : field . description . clone () , args , ty : field . ty . to_string () , deprecation : field . deprecation . clone () , cache_control : Default :: default () , external : field . external , requires : field . requires . clone () , provides : field . provides . clone () , visible : None , shareable : field . shareable , inaccessible : field . inaccessible , tags : field . tags . clone () , override_from : field . override_from . clone () , compute_complexity : None , directive_invocations : to_meta_directive_invocation (field . directives . clone ()) , requires_scopes : field . requires_scopes . clone () , } ,) ; } registry . types . insert (self . name . clone () , MetaType :: Interface { name : self . name . clone () , description : self . description . clone () , fields , possible_types : Default :: default () , extends : self . extends , keys : if ! self . keys . is_empty () { Some (self . keys . clone ()) } else { None } , visible : None , inaccessible : self . inaccessible , tags : self . tags . clone () , rust_typename : None , directive_invocations : to_meta_directive_invocation (self . directives . clone ()) , requires_scopes : self . requires_scopes . clone () , } ,) ; Ok (()) } }
    };
}

impl_450!();