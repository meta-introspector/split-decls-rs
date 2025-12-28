macro_rules! get_der_key {
    () => {
        pub (crate) fn get_der_key < D : Digest + BlockSizeUser > (key : & [u8]) -> Block < D > { let mut der_key = Block :: < D > :: default () ; if key . len () <= der_key . len () { der_key [.. key . len ()] . copy_from_slice (key) ; } else { let hash = D :: digest (key) ; if hash . len () <= der_key . len () { der_key [.. hash . len ()] . copy_from_slice (& hash) ; } else { let n = der_key . len () ; der_key . copy_from_slice (& hash [.. n]) ; } } der_key }
    };
}

get_der_key!()