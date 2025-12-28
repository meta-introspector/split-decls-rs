macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < T > ConstantTimeEq for NonZero < T > where T : ConstantTimeEq + ? Sized , { fn ct_eq (& self , other : & Self) -> Choice { self . 0 . ct_eq (& other . 0) } }
    };
}

impl_194!()