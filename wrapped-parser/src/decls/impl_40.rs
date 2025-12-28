macro_rules! deps {
    () => {
        Type!();
        BaseType!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Type { # [doc = " Create a type from the type string."] # [must_use] pub fn new (ty : & str) -> Option < Self > { let (nullable , ty) = if let Some (rest) = ty . strip_suffix ('!') { (false , rest) } else { (true , ty) } ; Some (Self { base : if let Some (ty) = ty . strip_prefix ('[') { BaseType :: List (Box :: new (Self :: new (ty . strip_suffix (']') ?) ?)) } else { BaseType :: Named (Name :: new (ty)) } , nullable , }) } }
    };
}

impl_40!()