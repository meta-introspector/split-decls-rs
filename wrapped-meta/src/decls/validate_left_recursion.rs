macro_rules! deps {
    () => {
        ParserRule!();
        Rule!();
    };
}

macro_rules! validate_left_recursion {
    () => {
        deps!();
        fn validate_left_recursion < 'a , 'i : 'a > (rules : & 'a [ParserRule < 'i >]) -> Vec < Error < Rule > > { left_recursion (to_hash_map (rules)) }
    };
}

validate_left_recursion!();