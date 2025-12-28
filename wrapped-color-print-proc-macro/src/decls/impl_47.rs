macro_rules! deps {
    () => {
        ColorTag!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a > PartialEq for ColorTag < 'a > { fn eq (& self , other : & ColorTag < 'a >) -> bool { and ! (self . source == other . source , self . is_close == other . is_close , self . change_set == other . change_set ,) } }
    };
}

impl_47!();