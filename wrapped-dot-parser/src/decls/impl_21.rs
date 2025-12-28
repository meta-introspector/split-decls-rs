macro_rules! deps {
    () => {
        Graphs!();
        ID!();
        PestError!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a str > for Graphs < (ID < 'a > , ID < 'a >) > { type Error = PestError ; fn try_from (s : & 'a str) -> Result < Self , PestError > { DotParser :: parse (Rule :: dotfile , s) . map (| mut p | match p . next () { None => { panic ! ("The toplevel `Pairs` is empty.") } Some (pair) => match Graphs :: try_from (pair) { Ok (g) => g , Err (e) => panic ! ("{}" , e) , } , }) } }
    };
}

impl_21!()