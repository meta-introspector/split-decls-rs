macro_rules! deps {
    () => {
        GitReference!();
        PrettyRef!();
        Result!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a > std :: fmt :: Display for PrettyRef < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let value : & str ; match self . inner { GitReference :: Branch (s) => { write ! (f , "branch=") ? ; value = s ; } GitReference :: Tag (s) => { write ! (f , "tag=") ? ; value = s ; } GitReference :: Rev (s) => { write ! (f , "rev=") ? ; value = s ; } GitReference :: DefaultBranch => unreachable ! () , } if self . url_encoded { for value in url :: form_urlencoded :: byte_serialize (value . as_bytes ()) { write ! (f , "{value}") ? ; } } else { write ! (f , "{value}") ? ; } Ok (()) } }
    };
}

impl_34!()