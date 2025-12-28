macro_rules! deps {
    () => {
        Stage!();
        State!();
    };
}

macro_rules! impls {
    () => {
        deps!();
        mod impls { use std :: fmt :: { Debug , Formatter } ; use crate :: { entry :: Stage , State } ; impl Debug for State { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { for entry in & self . entries { writeln ! (f , "{} {}{:?} {} {}" , match entry . flags . stage () { Stage :: Unconflicted => "       " , Stage :: Base => "BASE   " , Stage :: Ours => "OURS   " , Stage :: Theirs => "THEIRS " , } , if entry . flags . is_empty () { "" . to_string () } else { format ! ("{:?} " , entry . flags) } , entry . mode , entry . id , entry . path (self)) ? ; } Ok (()) } } }
    };
}

impls!();