macro_rules! deps {
    () => {
        ID!();
        PestError!();
        Graph!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a str > for Graph < (ID < 'a > , ID < 'a >) > { type Error = PestError ; fn try_from (s : & 'a str) -> Result < Self , PestError > { let mut pairs = DotParser :: parse (Rule :: dotgraph , s) ? ; match pairs . next () { None => { panic ! ("The toplevel `Pairs` is empty.") } Some (pair) => match Graph :: try_from (pair) { Ok (g) => Ok (g) , Err (e) => { panic ! ("{}" , e) ; } } , } } }
    };
}

impl_17!()