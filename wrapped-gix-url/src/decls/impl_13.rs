macro_rules! deps {
    () => {
        Url!();
        Scheme!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Default for Url { fn default () -> Self { Url { serialize_alternative_form : false , scheme : Scheme :: Ssh , user : None , password : None , host : None , port : None , path : bstr :: BString :: default () , } } }
    };
}

impl_13!()