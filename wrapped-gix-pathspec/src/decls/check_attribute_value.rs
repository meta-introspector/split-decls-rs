macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! check_attribute_value {
    () => {
        deps!();
        fn check_attribute_value (input : & BStr) -> Result < () , Error > { match input . iter () . copied () . find (| b | ! is_valid_attr_value (* b)) { Some (b) => Err (Error :: InvalidAttributeValue { character : b as char }) , None => Ok (()) , } }
    };
}

check_attribute_value!();