macro_rules! write {
    () => {
        # [allow (unused_macros)] macro_rules ! write { ($ ($ ignored : tt) *) => { compile_error ! ("use `self.print(value)` or `fmt::Trait::fmt(&value, self.out)`, \
             instead of `write!(self.out, \"{...}\", value)`") } ; }
    };
}

write!()