macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! IterFormatExt {
    () => {
        deps!();
        pub trait IterFormatExt : Iterator { fn format (self , separator : & str) -> Format < '_ , Self > where Self : Sized , { Format { sep : separator , inner : RefCell :: new (Some (self)) , } } }
    };
}

IterFormatExt!()