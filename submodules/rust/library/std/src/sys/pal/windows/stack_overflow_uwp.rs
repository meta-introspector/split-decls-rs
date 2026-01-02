
macro_rules! reserve_stack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reserve_stack in module {}", module_path!());
    };
}

mkfn!{
    reserve_stack_introspect!();
    pub fn reserve_stack () { }
}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub fn init () { }
}