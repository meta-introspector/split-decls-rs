macro_rules! deps {
    () => {
        Registry!();
    };
}

macro_rules! TypeDirective {
    () => {
        deps!();
        # [doc (hidden)] pub trait TypeDirective { fn name (& self) -> Cow < 'static , str > ; fn register (& self , registry : & mut Registry) ; }
    };
}

TypeDirective!();