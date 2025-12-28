macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! ContextStack {
    () => {
        deps!();
        # [doc = " Coroutine managing environment"] pub struct ContextStack { pub (crate) root : * mut Context , }
    };
}

ContextStack!()