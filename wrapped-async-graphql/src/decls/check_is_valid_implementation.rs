macro_rules! deps {
    () => {
        SchemaError!();
        Result!();
        Field!();
        BaseContainer!();
        Interface!();
    };
}

macro_rules! check_is_valid_implementation {
    () => {
        deps!();
        fn check_is_valid_implementation (implementing_type : & impl BaseContainer , implemented_type : & Interface ,) -> Result < () , SchemaError > { for field in implemented_type . fields . values () { let impl_field = implementing_type . field (& field . name) . ok_or_else (| | { format ! ("{} \"{}\" requires field \"{}\" defined by interface \"{}\"" , implementing_type . graphql_type () , implementing_type . name () , field . name , implemented_type . name) }) ? ; for arg in field . arguments . values () { let impl_arg = match impl_field . argument (& arg . name) { Some (impl_arg) => impl_arg , None if ! arg . ty . is_nullable () => { return Err (format ! ("Field \"{}.{}\" requires argument \"{}\" defined by interface \"{}.{}\"" , implementing_type . name () , field . name , arg . name , implemented_type . name , field . name ,) . into ()) ; } None => continue , } ; if ! arg . ty . is_subtype (& impl_arg . ty) { return Err (format ! ("Argument \"{}.{}.{}\" is not sub-type of \"{}.{}.{}\"" , implemented_type . name , field . name , arg . name , implementing_type . name () , field . name , arg . name) . into ()) ; } } if ! impl_field . ty () . is_subtype (& field . ty) { return Err (format ! ("Field \"{}.{}\" is not sub-type of \"{}.{}\"" , implementing_type . name () , field . name , implemented_type . name , field . name ,) . into ()) ; } } Ok (()) }
    };
}

check_is_valid_implementation!();