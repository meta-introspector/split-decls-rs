macro_rules! deps {
    () => {
        Html!();
        CliReport!();
        BencherReport!();
    };
}

macro_rules! Reports {
    () => {
        deps!();
        pub (crate) struct Reports { pub (crate) cli_enabled : bool , pub (crate) cli : CliReport , pub (crate) bencher_enabled : bool , pub (crate) bencher : BencherReport , pub (crate) csv_enabled : bool , pub (crate) html : Option < Html > , }
    };
}

Reports!()