macro_rules! deps {
    () => {
        SubstitutionTable!();
        Result!();
        Parse!();
        IndexStr!();
        ParseContext!();
    };
}

macro_rules! one_or_more {
    () => {
        deps!();
        fn one_or_more < 'a , 'b , P > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Vec < P > , IndexStr < 'b >) > where P : Parse , { let (first , mut tail) = P :: parse (ctx , subs , input) ? ; let mut results = vec ! [first] ; loop { if let Ok ((parsed , tail_tail)) = try_recurse ! (P :: parse (ctx , subs , tail)) { results . push (parsed) ; tail = tail_tail ; } else { return Ok ((results , tail)) ; } } }
    };
}

one_or_more!();