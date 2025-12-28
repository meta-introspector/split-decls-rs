macro_rules! deps {
    () => {
        CustomDirective!();
        Registry!();
        ServerResult!();
        ContextDirective!();
        Directive!();
    };
}

macro_rules! CustomDirectiveFactory {
    () => {
        deps!();
        # [doc (hidden)] pub trait CustomDirectiveFactory : Send + Sync + 'static { fn name (& self) -> Cow < 'static , str > ; fn register (& self , registry : & mut Registry) ; fn create (& self , ctx : & ContextDirective < '_ > , directive : & Directive ,) -> ServerResult < Box < dyn CustomDirective > > ; }
    };
}

CustomDirectiveFactory!();