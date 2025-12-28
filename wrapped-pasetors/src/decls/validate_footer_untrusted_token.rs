macro_rules! deps {
    () => {
        Error!();
        UntrustedToken!();
    };
}

macro_rules! validate_footer_untrusted_token {
    () => {
        deps!();
        # [doc = " If a footer is present, this is validated against the supplied."] pub (crate) fn validate_footer_untrusted_token < T : Purpose < V > , V : Version > (token : & UntrustedToken < T , V > , footer : Option < & [u8] > ,) -> Result < () , Error > { if let Some (known_footer) = footer { if token . untrusted_footer () . is_empty () { return Err (Error :: TokenValidation) ; } if ! bool :: from (known_footer . ct_eq (token . untrusted_footer ())) { return Err (Error :: TokenValidation) ; } } Ok (()) }
    };
}

validate_footer_untrusted_token!()