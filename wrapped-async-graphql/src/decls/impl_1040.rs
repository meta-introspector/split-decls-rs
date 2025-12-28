macro_rules! deps {
    () => {
        InputObject!();
        Scalar!();
        MetaType!();
        Object!();
        Union!();
        MetaTypeId!();
        Interface!();
    };
}

macro_rules! impl_1040 {
    () => {
        deps!();
        impl MetaTypeId { fn create_fake_type (& self , rust_typename : & 'static str) -> MetaType { match self { MetaTypeId :: Scalar => MetaType :: Scalar { name : "" . to_string () , description : None , is_valid : None , visible : None , inaccessible : false , tags : vec ! [] , specified_by_url : None , directive_invocations : vec ! [] , requires_scopes : vec ! [] , } , MetaTypeId :: Object => MetaType :: Object { name : "" . to_string () , description : None , fields : Default :: default () , cache_control : Default :: default () , extends : false , shareable : false , resolvable : true , inaccessible : false , interface_object : false , tags : vec ! [] , keys : None , visible : None , is_subscription : false , rust_typename : Some (rust_typename) , directive_invocations : vec ! [] , requires_scopes : vec ! [] , } , MetaTypeId :: Interface => MetaType :: Interface { name : "" . to_string () , description : None , fields : Default :: default () , possible_types : Default :: default () , extends : false , inaccessible : false , tags : vec ! [] , keys : None , visible : None , rust_typename : Some (rust_typename) , directive_invocations : vec ! [] , requires_scopes : vec ! [] , } , MetaTypeId :: Union => MetaType :: Union { name : "" . to_string () , description : None , possible_types : Default :: default () , visible : None , inaccessible : false , tags : vec ! [] , rust_typename : Some (rust_typename) , directive_invocations : vec ! [] , } , MetaTypeId :: Enum => MetaType :: Enum { name : "" . to_string () , description : None , enum_values : Default :: default () , visible : None , inaccessible : false , tags : vec ! [] , rust_typename : Some (rust_typename) , directive_invocations : vec ! [] , requires_scopes : vec ! [] , } , MetaTypeId :: InputObject => MetaType :: InputObject { name : "" . to_string () , description : None , input_fields : Default :: default () , visible : None , inaccessible : false , tags : vec ! [] , rust_typename : Some (rust_typename) , oneof : false , directive_invocations : vec ! [] , } , } } }
    };
}

impl_1040!()