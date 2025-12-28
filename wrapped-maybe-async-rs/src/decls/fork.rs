macro_rules! fork {
    () => {
        macro_rules ! fork { ($ fork : ident = $ input : ident) => { { $ fork = $ input . fork () ; &$ fork } } ; }
    };
}

fork!()