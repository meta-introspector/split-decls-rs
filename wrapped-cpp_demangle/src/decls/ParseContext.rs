macro_rules! deps {
    () => {
        ParseContextState!();
    };
}

macro_rules! ParseContext {
    () => {
        deps!();
        # [doc = " Common context needed when parsing."] # [derive (Debug , Clone)] pub struct ParseContext { max_recursion : u32 , state : Cell < ParseContextState > , }
    };
}

ParseContext!()