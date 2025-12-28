macro_rules! deps {
    () => {
        Delegate!();
        Error!();
        Spec!();
        Note!();
        Options!();
        Repository!();
    };
}

macro_rules! impl_879 {
    () => {
        deps!();
        impl < 'repo > Spec < 'repo > { # [doc = " Parse `spec` and use information from `repo` to resolve it, using `opts` to learn how to deal with ambiguity."] # [doc = ""] # [doc = " Note that it's easier and to use [`repo.rev_parse()`][Repository::rev_parse()] instead."] pub fn from_bstr < 'a > (spec : impl Into < & 'a BStr > , repo : & 'repo Repository , opts : Options) -> Result < Self , Error > { let mut delegate = Delegate :: new (repo , opts) ; match gix_revision :: spec :: parse (spec . into () , & mut delegate) { Err (parse :: Error :: Delegate) => Err (delegate . into_err ()) , Err (err) => Err (err . into ()) , Ok (()) => delegate . into_rev_spec () , } } }
    };
}

impl_879!()