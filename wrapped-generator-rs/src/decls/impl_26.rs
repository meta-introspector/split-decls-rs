macro_rules! deps {
    () => {
        Error!();
        GeneratorImpl!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < A , T > Drop for GeneratorImpl < '_ , A , T > { fn drop (& mut self) { if thread :: panicking () { return ; } if ! self . is_started () { return ; } if ! self . is_done () { trace ! ("generator is not done while drop") ; self . raw_cancel () } assert ! (self . is_done ()) ; let (total_stack , used_stack) = self . stack_usage () ; if used_stack < total_stack { } else { error ! ("stack overflow detected!") ; panic :: panic_any (Error :: StackErr) ; } } }
    };
}

impl_26!();