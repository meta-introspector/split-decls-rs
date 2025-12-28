macro_rules! deps {
    () => {
        ArgGroup!();
        Id!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        # [doc = " # Reflection"] impl ArgGroup { # [doc = " Get the name of the group"] # [inline] pub fn get_id (& self) -> & Id { & self . id } # [doc = " Reports whether [`ArgGroup::required`] is set"] # [inline] pub fn is_required_set (& self) -> bool { self . required } }
    };
}

impl_62!()