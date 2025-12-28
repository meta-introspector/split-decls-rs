macro_rules! deps {
    () => {
        Config!();
        Allow!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl Config { # [doc = " Allow everything which usually relates to a fully trusted environment"] pub fn all () -> Self { Config { git_binary : false , system : true , git : true , user : true , env : true , includes : true , } } # [doc = " Load only configuration local to the git repository."] pub fn isolated () -> Self { Config { git_binary : false , system : false , git : false , user : false , env : false , includes : false , } } }
    };
}

impl_505!()