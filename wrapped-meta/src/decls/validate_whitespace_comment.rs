macro_rules! deps {
    () => {
        Rule!();
        ParserRule!();
    };
}

macro_rules! validate_whitespace_comment {
    () => {
        deps!();
        fn validate_whitespace_comment < 'a , 'i : 'a > (rules : & 'a [ParserRule < 'i >]) -> Vec < Error < Rule > > { let map = to_hash_map (rules) ; rules . iter () . filter_map (| rule | { if rule . name == "WHITESPACE" || rule . name == "COMMENT" { if is_non_failing (& rule . node . expr , & map , & mut vec ! []) { Some (Error :: new_from_span (ErrorVariant :: CustomError { message : format ! ("{} cannot fail and will repeat infinitely" , & rule . name) , } , rule . node . span ,)) } else if is_non_progressing (& rule . node . expr , & map , & mut vec ! []) { Some (Error :: new_from_span (ErrorVariant :: CustomError { message : format ! ("{} is non-progressing and will repeat infinitely" , & rule . name) , } , rule . node . span ,)) } else { None } } else { None } }) . collect () }
    };
}

validate_whitespace_comment!()