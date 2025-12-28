macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl From < Env > for Vec < (String , String) > { fn from (env : Env) -> Vec < (String , String) > { let mut entries : Vec < _ > = env . entries . into_iter () . collect () ; entries . sort () ; entries } }
    };
}

impl_57!();