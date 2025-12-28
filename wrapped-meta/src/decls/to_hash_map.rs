macro_rules! deps {
    () => {
        ParserNode!();
        ParserRule!();
    };
}

macro_rules! to_hash_map {
    () => {
        deps!();
        fn to_hash_map < 'a , 'i : 'a > (rules : & 'a [ParserRule < 'i >]) -> HashMap < String , & 'a ParserNode < 'i > > { rules . iter () . map (| r | (r . name . clone () , & r . node)) . collect () }
    };
}

to_hash_map!();