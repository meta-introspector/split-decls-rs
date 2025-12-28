macro_rules! deps {
    () => {
        Item!();
        Kind!();
    };
}

macro_rules! gen_for_enum {
    () => {
        deps!();
        pub (crate) fn gen_for_enum (item : & Item , item_name : & Ident , variants : & [(& Variant , Item)] ,) -> Result < TokenStream , syn :: Error > { if ! matches ! (&* item . kind () , Kind :: Value) { abort ! { item . kind () . span () , "`{}` cannot be used with `value`" , item . kind () . name () , } } let lits = lits (variants) ? ; let value_variants = gen_value_variants (& lits) ; let to_possible_value = gen_to_possible_value (item , & lits) ; Ok (quote ! { # [allow (dead_code , unreachable_code , unused_variables , unused_braces , unused_qualifications ,)] # [allow (clippy :: style , clippy :: complexity , clippy :: pedantic , clippy :: restriction , clippy :: perf , clippy :: deprecated , clippy :: nursery , clippy :: cargo , clippy :: suspicious_else_formatting , clippy :: almost_swapped , clippy :: redundant_locals ,)] # [automatically_derived] impl clap :: ValueEnum for # item_name { # value_variants # to_possible_value } }) }
    };
}

gen_for_enum!()