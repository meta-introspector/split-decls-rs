macro_rules! deps {
    () => {
        Issue!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The error returned [outcome validation](match_lhs::Outcome::validated())."] # [derive (Debug)] pub struct Error { # [doc = " All issues discovered during validation."] pub issues : Vec < Issue > , }
    };
}

Error!()