macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Validate {
    () => {
        deps!();
        # [doc = " Provide a way to validate a value, or decode a value from `git-config`."] pub trait Validate { # [doc = " Validate `value` or return an error."] fn validate (& self , value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > ; }
    };
}

Validate!();