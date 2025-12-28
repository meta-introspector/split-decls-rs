macro_rules! take_read_internal {
    () => {
        fn take_read_internal < R : AsyncRead + ? Sized > (mut rd : Pin < & mut R > , cx : & mut Context < '_ > , buf : & mut [u8] , limit : & mut u64 ,) -> Poll < Result < usize > > { if * limit == 0 { return Poll :: Ready (Ok (0)) ; } let max = cmp :: min (buf . len () as u64 , * limit) as usize ; match ready ! (rd . as_mut () . poll_read (cx , & mut buf [.. max])) { Ok (n) => { * limit -= n as u64 ; Poll :: Ready (Ok (n)) } Err (e) => Poll :: Ready (Err (e)) , } }
    };
}

take_read_internal!();