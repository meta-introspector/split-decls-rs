macro_rules! deps {
    () => {
        Key!();
        Properties!();
        Configure!();
        Figure!();
        Default!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl Configure < Key > for Figure { type Properties = key :: Properties ; # [doc = " Configures the key (legend)"] fn configure < F > (& mut self , _ : Key , configure : F) -> & mut Figure where F : FnOnce (& mut key :: Properties) -> & mut key :: Properties , { if self . key . is_some () { configure (self . key . as_mut () . unwrap ()) ; } else { let mut key = Default :: default () ; configure (& mut key) ; self . key = Some (key) ; } self } }
    };
}

impl_127!()