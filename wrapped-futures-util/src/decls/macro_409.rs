macro_rules! macro_409 {
    () => {
        pin_project ! { # [doc = " Future for the [`Peekable::peek_mut`](self::Peekable::peek_mut) method."] # [must_use = "futures do nothing unless polled"] pub struct PeekMut <'a , St : Stream > { inner : Option < Pin <&'a mut Peekable < St >>>, } }
    };
}

macro_409!();