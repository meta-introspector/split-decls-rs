macro_rules! deps {
    () => {
        Ready!();
        Flush!();
    };
}

macro_rules! impl_1081 {
    () => {
        deps!();
        impl < W : AsyncWrite > LineWriter < W > { # [doc = " Create a new `LineWriter` with default buffer capacity. The default is currently 1KB"] # [doc = " which was taken from `std::io::LineWriter`"] pub fn new (inner : W) -> Self { Self :: with_capacity (1024 , inner) } # [doc = " Creates a new `LineWriter` with the specified buffer capacity."] pub fn with_capacity (capacity : usize , inner : W) -> Self { Self { buf_writer : BufWriter :: with_capacity (capacity , inner) } } # [doc = " Flush `buf_writer` if last char is \"new line\""] fn flush_if_completed_line (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { let this = self . project () ; match this . buf_writer . buffer () . last () . copied () { Some (b'\n') => this . buf_writer . flush_buf (cx) , _ => Poll :: Ready (Ok (())) , } } # [doc = " Returns a reference to `buf_writer`'s internally buffered data."] pub fn buffer (& self) -> & [u8] { self . buf_writer . buffer () } # [doc = " Acquires a reference to the underlying sink or stream that this combinator is"] # [doc = " pulling from."] pub fn get_ref (& self) -> & W { self . buf_writer . get_ref () } }
    };
}

impl_1081!()