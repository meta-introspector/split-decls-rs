macro_rules! deps {
    () => {
        Delimited!();
    };
}

macro_rules! IterDelimited {
    () => {
        deps!();
        pub trait IterDelimited : Iterator + Sized { fn delimited (self) -> Delimited < Self > { Delimited { is_first : true , iter : self . peekable () , } } }
    };
}

IterDelimited!();