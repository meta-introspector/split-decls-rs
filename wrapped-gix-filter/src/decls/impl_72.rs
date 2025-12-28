macro_rules! deps {
    () => {
        Error!();
        ReadProcessOutputAndStatus!();
        Process!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl std :: io :: Read for ReadProcessOutputAndStatus < '_ > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let num_read = self . inner . read (buf) ? ; if num_read == 0 { self . inner . reset_with (& [gix_packetline :: PacketLineRef :: Flush]) ; let status = read_status (& mut self . inner) ? ; if status . is_success () { Ok (0) } else { Err (std :: io :: Error :: other (format ! ("Process indicated error after reading: {}" , status . message () . unwrap_or_default ()))) } } else { Ok (num_read) } } }
    };
}

impl_72!()