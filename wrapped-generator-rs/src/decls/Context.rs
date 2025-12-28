macro_rules! deps {
    () => {
        RegContext!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " generator context"] # [repr (C)] # [repr (align (128))] pub struct Context { # [doc = " generator regs context"] pub regs : RegContext , # [doc = " child context"] child : * mut Context , # [doc = " parent context"] pub parent : * mut Context , # [doc = " passed in para for send"] pub para : MaybeUninit < * mut dyn Any > , # [doc = " this is just a buffer for the return value"] pub ret : MaybeUninit < * mut dyn Any > , # [doc = " track generator ref, yield will -1, send will +1"] pub _ref : usize , # [doc = " context local storage"] pub local_data : * mut u8 , # [doc = " propagate panic"] pub err : Option < Box < dyn Any + Send > > , # [doc = " cached stack guard for fast path"] pub stack_guard : (usize , usize) , }
    };
}

Context!()