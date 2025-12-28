macro_rules! deps {
    () => {
        Field!();
        ContextBase!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " Context object for resolve field"] pub type Context < 'a > = ContextBase < 'a , & 'a Positioned < Field > > ;
    };
}

Context!();