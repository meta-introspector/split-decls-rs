macro_rules! deps {
    () => {
        Init!();
        Section!();
        Any!();
    };
}

macro_rules! impl_758 {
    () => {
        deps!();
        # [doc = " Init"] impl Any < validate :: All > { # [doc = " Create a new instance from `name` and `section`"] pub const fn new (name : & 'static str , section : & 'static dyn Section) -> Self { Any :: new_with_validate (name , section , validate :: All) } }
    };
}

impl_758!()