macro_rules! deps {
    () => {
        Defaultness!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl Defaultness { pub fn has_value (& self) -> bool { match * self { Defaultness :: Default { has_value } => has_value , Defaultness :: Final => true , } } pub fn is_final (& self) -> bool { * self == Defaultness :: Final } pub fn is_default (& self) -> bool { matches ! (* self , Defaultness :: Default { .. }) } }
    };
}

impl_293!()