macro_rules! deps {
    () => {
        Message!();
        IoThreads!();
    };
}

macro_rules! socket_transport {
    () => {
        deps!();
        pub (crate) fn socket_transport (stream : TcpStream ,) -> (Sender < Message > , Receiver < Message > , IoThreads) { let (reader_receiver , reader) = make_reader (stream . try_clone () . unwrap ()) ; let (writer_sender , writer , messages_to_drop) = make_write (stream) ; let dropper = std :: thread :: spawn (move | | { messages_to_drop . into_iter () . for_each (drop) ; }) ; let io_threads = make_io_threads (reader , writer , dropper) ; (writer_sender , reader_receiver , io_threads) }
    };
}

socket_transport!()