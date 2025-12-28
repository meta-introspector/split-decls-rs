macro_rules! deps {
    () => {
        Clone!();
        Default!();
        Replace!();
    };
}

macro_rules! Rewrite {
    () => {
        deps!();
        # [derive (Default , Debug , Clone)] pub (crate) struct Rewrite { url_rewrite : Vec < Replace > , push_url_rewrite : Vec < Replace > , }
    };
}

Rewrite!();