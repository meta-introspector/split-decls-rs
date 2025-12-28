macro_rules! Deprecation {
    () => {
        # [derive (Debug , Clone , Default)] pub enum Deprecation { # [default] NoDeprecated , Deprecated { reason : Option < String > , } , }
    };
}

Deprecation!()