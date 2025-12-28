macro_rules! macro_400 {
    () => {
        pin_project ! { # [doc = " A `Stream` that implements a `peek` method."] # [doc = ""] # [doc = " The `peek` method can be used to retrieve a reference"] # [doc = " to the next `Stream::Item` if available. A subsequent"] # [doc = " call to `poll` will return the owned item."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Peekable < St : Stream > { # [pin] stream : Fuse < St >, peeked : Option < St :: Item >, } }
    };
}

macro_400!()