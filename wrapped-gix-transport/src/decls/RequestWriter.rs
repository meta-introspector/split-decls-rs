macro_rules! deps {
    () => {
        ExtendedBufRead!();
        MessageKind!();
    };
}

macro_rules! RequestWriter {
    () => {
        deps!();
        # [doc = " A [`Write`][io::Write] implementation optimized for writing packet lines."] # [doc = " A type implementing `Write` for packet lines, which when done can be transformed into a `Read` for"] # [doc = " obtaining the response."] pub struct RequestWriter < 'a > { on_into_read : MessageKind , writer : Writer < Box < dyn io :: Write + 'a > > , reader : Box < dyn ExtendedBufRead < 'a > + Unpin + 'a > , trace : bool , }
    };
}

RequestWriter!()