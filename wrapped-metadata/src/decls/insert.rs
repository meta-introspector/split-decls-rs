macro_rules! deps {
    () => {
        HashType!();
        Item!();
    };
}

macro_rules! insert {
    () => {
        deps!();
        fn insert < 'a > (members : & mut HashType < 'a > , namespace : & 'a str , name : & 'a str , member : Item < 'a >) { members . entry (namespace) . or_default () . entry (name) . or_default () . push (member) ; }
    };
}

insert!();