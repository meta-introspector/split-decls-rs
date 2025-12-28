macro_rules! deps {
    () => {
        Stack!();
        RegContext!();
        Context!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl RegContext { pub fn empty () -> RegContext { RegContext { regs : Registers :: new () , } } # [inline] pub fn prefetch (& self) { self . regs . prefetch () ; } # [doc = " Create a new context, only used in tests"] # [cfg (test)] fn new (init : InitFn , arg : usize , start : * mut usize , stack : & Stack) -> RegContext { let mut ctx = RegContext :: empty () ; ctx . init_with (init , arg , start , stack) ; ctx } # [doc = " init the generator register"] # [inline] pub fn init_with (& mut self , init : InitFn , arg : usize , start : * mut usize , stack : & Stack) { initialize_call_frame (& mut self . regs , init , arg , start , stack) ; } # [doc = " Switch contexts"] # [doc = ""] # [doc = " Suspend the current execution context and resume another by"] # [doc = " saving the registers values of the executing thread to a Context"] # [doc = " then loading the registers from a previously saved Context."] # [inline] pub fn swap (out_context : & mut RegContext , in_context : & RegContext) { unsafe { swap_registers (& mut out_context . regs , & in_context . regs) } } # [doc = " Load the context and switch. This function will never return."] # [inline] # [cfg (test)] pub fn load (to_context : & RegContext) { let mut cur = Registers :: new () ; let regs : & Registers = & to_context . regs ; unsafe { swap_registers (& mut cur , regs) } } }
    };
}

impl_29!()