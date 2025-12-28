macro_rules! deps {
    () => {
        User!();
        Allow!();
    };
}

macro_rules! impl_981 {
    () => {
        deps!();
        impl Allow { # [doc = " Return true if we represent something like 'allow == true'."] pub fn to_bool (self , user_allowed : Option < bool >) -> bool { match self { Allow :: Always => true , Allow :: Never => false , Allow :: User => user_allowed . unwrap_or (true) , } } }
    };
}

impl_981!()