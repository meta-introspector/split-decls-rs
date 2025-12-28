macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! read_until_internal {
    () => {
        deps!();
        pub (super) fn read_until_internal < R : AsyncBufRead + ? Sized > (mut reader : Pin < & mut R > , cx : & mut Context < '_ > , byte : u8 , buf : & mut Vec < u8 > , read : & mut usize ,) -> Poll < io :: Result < usize > > { loop { let (done , used) = { let available = ready ! (reader . as_mut () . poll_fill_buf (cx)) ? ; if let Some (i) = memchr :: memchr (byte , available) { buf . extend_from_slice (& available [..= i]) ; (true , i + 1) } else { buf . extend_from_slice (available) ; (false , available . len ()) } } ; reader . as_mut () . consume (used) ; * read += used ; if done || used == 0 { return Poll :: Ready (Ok (* read)) ; } } }
    };
}

read_until_internal!()