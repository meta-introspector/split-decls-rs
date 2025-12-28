macro_rules! deps {
    () => {
        Directive!();
    };
}

macro_rules! impl_directive {
    () => {
        deps!();
        macro_rules ! impl_directive { () => { # [doc = " Attach directive to the entity"] # [inline] pub fn directive (mut self , directive : Directive) -> Self { self . directives . push (directive) ; self } } ; }
    };
}

impl_directive!();