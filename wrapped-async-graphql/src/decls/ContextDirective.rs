macro_rules! deps {
    () => {
        Directive!();
        Context!();
        ContextBase!();
    };
}

macro_rules! ContextDirective {
    () => {
        deps!();
        # [doc = " Context object for execute directive."] pub type ContextDirective < 'a > = ContextBase < 'a , & 'a Positioned < Directive > > ;
    };
}

ContextDirective!();