macro_rules! deps {
    () => {
        Attributes!();
        Clone!();
        Environment!();
        Config!();
    };
}

macro_rules! Permissions {
    () => {
        deps!();
        # [doc = " Permissions associated with various resources of a git repository"] # [derive (Debug , Clone)] pub struct Permissions { # [doc = " Control which environment variables may be accessed."] pub env : permissions :: Environment , # [doc = " Permissions related where git configuration should be loaded from."] pub config : permissions :: Config , # [doc = " Permissions related to where `gitattributes` should be loaded from."] pub attributes : permissions :: Attributes , }
    };
}

Permissions!();