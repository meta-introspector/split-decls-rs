macro_rules! deps {
    () => {
        NextIfEqFn!();
    };
}

macro_rules! macro_417 {
    () => {
        deps!();
        pin_project ! { # [doc = " Future for the [`Peekable::next_if_eq`](self::Peekable::next_if_eq) method."] # [must_use = "futures do nothing unless polled"] pub struct NextIfEq <'a , St : Stream , T : ? Sized > { # [pin] inner : NextIf <'a , St , NextIfEqFn <'a , T , St :: Item >>, } }
    };
}

macro_417!();