macro_rules! deps {
    () => {
        Reader!();
        TypeName!();
    };
}

macro_rules! get_type_name {
    () => {
        deps!();
        fn get_type_name (reader : & Reader , path : & str) -> TypeName { if let Some ((namespace , name)) = path . rsplit_once ('.') { if let Some ((namespace , types)) = reader . get_key_value (namespace) { if let Some ((name , _)) = types . get_key_value (name) { return TypeName (namespace , name) ; } } } else { for (namespace , types) in reader . iter () { if let Some ((name , _)) = types . get_key_value (path) { return TypeName (namespace , name) ; } } } panic ! ("type not found: `{path}`") ; }
    };
}

get_type_name!();