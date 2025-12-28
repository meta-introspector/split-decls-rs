macro_rules! deps {
    () => {
        Curl!();
        Options!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl Default for Curl { fn default () -> Self { let (handle , req , res) = remote :: new () ; Curl { handle : Some (handle) , req , res , config : http :: Options :: default () , } } }
    };
}

impl_63!()