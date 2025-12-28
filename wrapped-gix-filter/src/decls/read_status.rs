macro_rules! deps {
    () => {
        PacketlineReader!();
        Status!();
    };
}

macro_rules! read_status {
    () => {
        deps!();
        fn read_status (read : & mut PacketlineReader < '_ >) -> std :: io :: Result < process :: Status > { let mut status = process :: Status :: Previous ; let mut buf = String :: new () ; let mut count = 0 ; loop { buf . clear () ; let num_read = read . read_line_to_string (& mut buf) ? ; if num_read == 0 { break ; } if let Some (name) = buf . strip_prefix ("status=") { status = process :: Status :: Named (name . trim_end () . into ()) ; } count += 1 ; } if count > 0 && matches ! (status , process :: Status :: Previous) { status = process :: Status :: Unset ; } read . reset_with (& [gix_packetline :: PacketLineRef :: Flush]) ; Ok (status) }
    };
}

read_status!()