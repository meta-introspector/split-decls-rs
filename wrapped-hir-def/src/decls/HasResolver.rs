macro_rules! deps {
    () => {
        DefDatabase!();
        Resolver!();
    };
}

macro_rules! HasResolver {
    () => {
        deps!();
        pub trait HasResolver : Copy { # [doc = " Builds a resolver for type references inside this def."] fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > ; }
    };
}

HasResolver!();