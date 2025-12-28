macro_rules! deps {
    () => {
        MetaTypeName!();
    };
}

macro_rules! impl_1029 {
    () => {
        deps!();
        impl MetaTypeName < '_ > { # [inline] pub fn create (type_name : & str) -> MetaTypeName { if let Some (type_name) = type_name . strip_suffix ('!') { MetaTypeName :: NonNull (type_name) } else if let Some (type_name) = strip_brackets (type_name) { MetaTypeName :: List (type_name) } else { MetaTypeName :: Named (type_name) } } # [inline] pub fn concrete_typename (type_name : & str) -> & str { match MetaTypeName :: create (type_name) { MetaTypeName :: List (type_name) => Self :: concrete_typename (type_name) , MetaTypeName :: NonNull (type_name) => Self :: concrete_typename (type_name) , MetaTypeName :: Named (type_name) => type_name , } } # [inline] pub fn is_non_null (& self) -> bool { matches ! (self , MetaTypeName :: NonNull (_)) } # [inline] # [must_use] pub fn unwrap_non_null (& self) -> Self { match self { MetaTypeName :: NonNull (ty) => MetaTypeName :: create (ty) , _ => * self , } } # [inline] pub fn is_subtype (& self , sub : & MetaTypeName < '_ >) -> bool { match (self , sub) { (MetaTypeName :: NonNull (super_type) , MetaTypeName :: NonNull (sub_type)) | (MetaTypeName :: Named (super_type) , MetaTypeName :: NonNull (sub_type)) => { MetaTypeName :: create (super_type) . is_subtype (& MetaTypeName :: create (sub_type)) } (MetaTypeName :: Named (super_type) , MetaTypeName :: Named (sub_type)) => { super_type == sub_type } (MetaTypeName :: List (super_type) , MetaTypeName :: List (sub_type)) => { MetaTypeName :: create (super_type) . is_subtype (& MetaTypeName :: create (sub_type)) } _ => false , } } # [inline] pub fn is_list (& self) -> bool { match self { MetaTypeName :: List (_) => true , MetaTypeName :: NonNull (ty) => MetaTypeName :: create (ty) . is_list () , MetaTypeName :: Named (name) => name . ends_with (']') , } } }
    };
}

impl_1029!();