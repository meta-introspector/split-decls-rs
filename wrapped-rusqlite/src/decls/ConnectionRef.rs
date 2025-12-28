macro_rules! deps {
    () => {
        Context!();
        Connection!();
    };
}

macro_rules! ConnectionRef {
    () => {
        deps!();
        # [doc = " A reference to a connection handle with a lifetime bound to context."] pub struct ConnectionRef < 'ctx > { conn : Connection , phantom : PhantomData < & 'ctx Context > , }
    };
}

ConnectionRef!()