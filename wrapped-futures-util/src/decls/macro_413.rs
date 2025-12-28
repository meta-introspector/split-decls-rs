macro_rules! macro_413 {
    () => {
        pin_project ! { # [doc = " Future for the [`Peekable::next_if`](self::Peekable::next_if) method."] # [must_use = "futures do nothing unless polled"] pub struct NextIf <'a , St : Stream , F > { inner : Option < (Pin <&'a mut Peekable < St >>, F) >, } }
    };
}

macro_413!();