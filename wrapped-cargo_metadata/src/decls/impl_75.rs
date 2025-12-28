macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Edition { # [doc = " Return the string representation of the edition"] pub fn as_str (& self) -> & 'static str { use Edition :: * ; match self { E2015 => "2015" , E2018 => "2018" , E2021 => "2021" , E2024 => "2024" , _E2027 => "2027" , _E2030 => "2030" , } } }
    };
}

impl_75!();