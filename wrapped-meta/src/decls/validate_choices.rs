macro_rules! deps {
    () => {
        ParserExpr!();
        Rule!();
        ParserRule!();
    };
}

macro_rules! validate_choices {
    () => {
        deps!();
        fn validate_choices < 'a , 'i : 'a > (rules : & 'a [ParserRule < 'i >]) -> Vec < Error < Rule > > { let mut result = vec ! [] ; let map = to_hash_map (rules) ; for rule in rules { let mut errors = rule . node . clone () . filter_map_top_down (| node | match node . expr { ParserExpr :: Choice (ref lhs , _) => { let node = match lhs . expr { ParserExpr :: Choice (_ , ref rhs) => rhs , _ => lhs , } ; if is_non_failing (& node . expr , & map , & mut vec ! []) { Some (Error :: new_from_span (ErrorVariant :: CustomError { message : "expression cannot fail; following choices cannot be reached" . to_owned () , } , node . span ,)) } else { None } } _ => None , }) ; result . append (& mut errors) ; } result }
    };
}

validate_choices!();