macro_rules! deps {
    () => {
        Buffer!();
        WriteStyle!();
        BufferWriter!();
        WritableTarget!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl BufferWriter { pub (crate) fn stderr (is_test : bool , write_style : WriteStyle) -> Self { BufferWriter { target : if is_test { WritableTarget :: PrintStderr } else { WritableTarget :: WriteStderr } , write_style , } } pub (crate) fn stdout (is_test : bool , write_style : WriteStyle) -> Self { BufferWriter { target : if is_test { WritableTarget :: PrintStdout } else { WritableTarget :: WriteStdout } , write_style , } } pub (crate) fn pipe (pipe : Box < Mutex < dyn io :: Write + Send + 'static > > , write_style : WriteStyle ,) -> Self { BufferWriter { target : WritableTarget :: Pipe (pipe) , write_style , } } pub (crate) fn write_style (& self) -> WriteStyle { self . write_style } pub (crate) fn buffer (& self) -> Buffer { Buffer (Vec :: new ()) } pub (crate) fn print (& self , buf : & Buffer) -> io :: Result < () > { # ! [allow (clippy :: print_stdout)] # ! [allow (clippy :: print_stderr)] use std :: io :: Write as _ ; let buf = buf . as_bytes () ; match & self . target { WritableTarget :: WriteStdout => { let stream = io :: stdout () ; # [cfg (feature = "color")] let stream = anstream :: AutoStream :: new (stream , self . write_style . into ()) ; let mut stream = stream . lock () ; stream . write_all (buf) ? ; stream . flush () ? ; } WritableTarget :: PrintStdout => { # [cfg (feature = "color")] let buf = adapt (buf , self . write_style) ? ; # [cfg (feature = "color")] let buf = & buf ; let buf = String :: from_utf8_lossy (buf) ; print ! ("{buf}") ; } WritableTarget :: WriteStderr => { let stream = io :: stderr () ; # [cfg (feature = "color")] let stream = anstream :: AutoStream :: new (stream , self . write_style . into ()) ; let mut stream = stream . lock () ; stream . write_all (buf) ? ; stream . flush () ? ; } WritableTarget :: PrintStderr => { # [cfg (feature = "color")] let buf = adapt (buf , self . write_style) ? ; # [cfg (feature = "color")] let buf = & buf ; let buf = String :: from_utf8_lossy (buf) ; eprint ! ("{buf}") ; } WritableTarget :: Pipe (pipe) => { # [cfg (feature = "color")] let buf = adapt (buf , self . write_style) ? ; # [cfg (feature = "color")] let buf = & buf ; let mut stream = pipe . lock () . expect ("no panics while held") ; stream . write_all (buf) ? ; stream . flush () ? ; } } Ok (()) } }
    };
}

impl_24!();