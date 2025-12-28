macro_rules! deps {
    () => {
        State!();
        PpAnn!();
    };
}

macro_rules! to_string {
    () => {
        deps!();
        fn to_string < F > (ann : & dyn PpAnn , f : F) -> String where F : FnOnce (& mut State < '_ >) , { let mut printer = State { s : pp :: Printer :: new () , comments : None , attrs : & | _ | & [] , ann } ; f (& mut printer) ; printer . s . eof () }
    };
}

to_string!()