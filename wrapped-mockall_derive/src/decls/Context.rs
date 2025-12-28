macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " Generates Context, which manages the context for expectations of static"] # [doc = " methods."] struct Context < 'a > { f : & 'a MockFunction }
    };
}

Context!();