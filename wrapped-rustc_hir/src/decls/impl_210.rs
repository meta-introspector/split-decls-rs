macro_rules! deps {
    () => {
        CoroutineDesugaring!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl fmt :: Display for CoroutineDesugaring { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CoroutineDesugaring :: Async => { if f . alternate () { f . write_str ("`async` ") ? ; } else { f . write_str ("async ") ? } } CoroutineDesugaring :: Gen => { if f . alternate () { f . write_str ("`gen` ") ? ; } else { f . write_str ("gen ") ? } } CoroutineDesugaring :: AsyncGen => { if f . alternate () { f . write_str ("`async gen` ") ? ; } else { f . write_str ("async gen ") ? } } } Ok (()) } }
    };
}

impl_210!()