macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! new_format_default {
    () => {
        deps!();
        pub fn new_format_default < I > (iter : I , separator : & str) -> Format < '_ , I > where I : Iterator , { Format { sep : separator , inner : Cell :: new (Some (iter)) , } }
    };
}

new_format_default!()