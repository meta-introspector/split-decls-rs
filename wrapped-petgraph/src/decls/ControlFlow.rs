macro_rules! deps {
    () => {
        Control!();
    };
}

macro_rules! ControlFlow {
    () => {
        deps!();
        # [doc = " Control flow for callbacks."] # [doc = ""] # [doc = " The empty return value `()` is equivalent to continue."] pub trait ControlFlow { fn continuing () -> Self ; fn should_break (& self) -> bool ; fn should_prune (& self) -> bool ; }
    };
}

ControlFlow!();