macro_rules! deps {
    () => {
        Result!();
        Parse!();
        IndexStr!();
        ParseContext!();
        SubstitutionTable!();
    };
}

macro_rules! zero_or_more {
    () => {
        deps!();
        fn zero_or_more < 'a , 'b , P > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Vec < P > , IndexStr < 'b >) > where P : Parse , { let mut tail = input ; let mut results = vec ! [] ; loop { if let Ok ((parsed , tail_tail)) = try_recurse ! (P :: parse (ctx , subs , tail)) { results . push (parsed) ; tail = tail_tail ; } else { return Ok ((results , tail)) ; } } }
    };
}

zero_or_more!();