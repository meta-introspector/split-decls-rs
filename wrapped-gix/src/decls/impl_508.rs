macro_rules! deps {
    () => {
        Allow!();
        Attributes!();
    };
}

macro_rules! impl_508 {
    () => {
        deps!();
        impl Attributes { # [doc = " Allow everything which usually relates to a fully trusted environment"] pub fn all () -> Self { Attributes { git_binary : false , system : true , git : true , } } # [doc = " Allow loading attributes that are local to the git repository."] pub fn isolated () -> Self { Attributes { git_binary : false , system : false , git : false , } } }
    };
}

impl_508!();