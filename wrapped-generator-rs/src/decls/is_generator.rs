macro_rules! deps {
    () => {
        ContextStack!();
    };
}

macro_rules! is_generator {
    () => {
        deps!();
        # [doc = " check the current context if it's generator"] # [inline] pub fn is_generator () -> bool { let env = ContextStack :: current () ; let root = unsafe { & mut * env . root } ; ! root . child . is_null () }
    };
}

is_generator!();