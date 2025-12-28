macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl Token { # [doc = " This is a valid PASETO secret key."] # [doc = ""] # [doc = " This one is already publicly available as part of the text of the RFC so is safe to use for tests."] pub fn rfc_key () -> Token { Token :: Keys ("k3.secret.fNYVuMvBgOlljt9TDohnaYLblghqaHoQquVZwgR6X12cBFHZLFsaU3q7X3k1Zn36" . to_string () , Some ("sub" . to_string ()) ,) } }
    };
}

impl_114!();