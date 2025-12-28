macro_rules! deps {
    () => {
        Thread!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " Thread factory, which can be used in order to configure the properties of"] # [doc = " a new thread."] # [derive (Debug)] pub struct Builder { name : Option < String > , stack_size : Option < usize > , }
    };
}

Builder!()