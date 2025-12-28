macro_rules! deps {
    () => {
        Primitive!();
    };
}

macro_rules! PublicFlags {
    () => {
        deps!();
        # [doc = " A trait for referencing the `bitflags`-owned internal type"] # [doc = " without exposing it publicly."] pub trait PublicFlags { # [doc = " The type of the underlying storage."] type Primitive : Primitive ; # [doc = " The type of the internal field on the generated flags type."] type Internal ; }
    };
}

PublicFlags!()