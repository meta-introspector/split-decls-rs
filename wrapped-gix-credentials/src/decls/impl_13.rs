macro_rules! deps {
    () => {
        Cascade!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Default for Cascade { fn default () -> Self { Cascade { programs : Vec :: new () , stderr : true , use_http_path : false , query_user_only : false , } } }
    };
}

impl_13!()