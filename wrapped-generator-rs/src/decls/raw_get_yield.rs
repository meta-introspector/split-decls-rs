macro_rules! deps {
    () => {
        Context!();
        Error!();
    };
}

macro_rules! raw_get_yield {
    () => {
        deps!();
        # [doc = " get the passed in para from context"] # [inline] fn raw_get_yield < A : Any > (context : & mut Context) -> Option < A > { if unlikely (! context . is_generator ()) { { error ! ("get yield from none generator context") ; std :: panic :: panic_any (Error :: ContextErr) ; } } context . get_para () }
    };
}

raw_get_yield!();