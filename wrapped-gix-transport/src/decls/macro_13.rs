macro_rules! deps {
    () => {
        MessageKind!();
        RequestWriter!();
        ExtendedBufRead!();
    };
}

macro_rules! macro_13 {
    () => {
        deps!();
        pin_project ! { # [doc = " A [`Write`][io::Write] implementation optimized for writing packet lines."] # [doc = " A type implementing `Write` for packet lines, which when done can be transformed into a `Read` for"] # [doc = " obtaining the response."] pub struct RequestWriter <'a > { on_into_read : MessageKind , # [pin] writer : Writer < Box < dyn AsyncWrite + Unpin + 'a >>, reader : Box < dyn ExtendedBufRead <'a > + Unpin + 'a >, trace : bool , } }
    };
}

macro_13!()