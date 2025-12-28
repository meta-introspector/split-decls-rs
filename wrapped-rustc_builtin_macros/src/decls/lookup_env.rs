macro_rules! lookup_env {
    () => {
        fn lookup_env < 'cx > (cx : & 'cx ExtCtxt < '_ > , var : Symbol) -> Result < Symbol , VarError > { let var = var . as_str () ; if let Some (value) = cx . sess . opts . logical_env . get (var) { return Ok (Symbol :: intern (value)) ; } Ok (Symbol :: intern (& env :: var (var) ?)) }
    };
}

lookup_env!()