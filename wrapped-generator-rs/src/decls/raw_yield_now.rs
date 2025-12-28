macro_rules! deps {
    () => {
        ContextStack!();
        RegContext!();
        Context!();
    };
}

macro_rules! raw_yield_now {
    () => {
        deps!();
        # [inline] pub fn raw_yield_now (env : & ContextStack , cur : & mut Context) { let parent = env . pop_context (cur as * mut _) ; RegContext :: swap (& mut cur . regs , & parent . regs) ; }
    };
}

raw_yield_now!();