macro_rules! InnerConnection {
    () => {
        # [derive (Debug)] struct InnerConnection { socket : TcpStream , receive_buffer : Vec < u8 > , send_buffer : Vec < u8 > , }
    };
}

InnerConnection!();