macro_rules! macro_405 {
    () => {
        pin_project ! { # [doc = " Future for the [`Peekable::peek`](self::Peekable::peek) method."] # [must_use = "futures do nothing unless polled"] pub struct Peek <'a , St : Stream > { inner : Option < Pin <&'a mut Peekable < St >>>, } }
    };
}

macro_405!();