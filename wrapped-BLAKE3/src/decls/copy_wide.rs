macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! copy_wide {
    () => {
        deps!();
        # [cfg (feature = "std")] pub (crate) fn copy_wide (mut reader : impl std :: io :: Read , hasher : & mut crate :: Hasher ,) -> std :: io :: Result < u64 > { let mut buffer = [0 ; 65536] ; let mut total = 0 ; loop { match reader . read (& mut buffer) { Ok (0) => return Ok (total) , Ok (n) => { hasher . update (& buffer [.. n]) ; total += n as u64 ; } Err (e) if e . kind () == std :: io :: ErrorKind :: Interrupted => continue , Err (e) => return Err (e) , } } }
    };
}

copy_wide!();