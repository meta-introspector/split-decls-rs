macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! canonical_script {
    () => {
        deps!();
        fn canonical_script (normalized_value : & str ,) -> Result < Option < & 'static str > , Error > { let scripts = property_values ("Script") ? . unwrap () ; Ok (canonical_value (scripts , normalized_value)) }
    };
}

canonical_script!();