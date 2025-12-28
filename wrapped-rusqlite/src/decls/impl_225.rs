macro_rules! deps {
    () => {
        Result!();
        Row!();
        Null!();
        ValueRef!();
        Blob!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        # [doc = " Debug `Row` like an ordered `Map<Result<&str>, Result<(Type, ValueRef)>>`"] # [doc = " with column name as key except that for `Type::Blob` only its size is"] # [doc = " printed (not its content)."] impl std :: fmt :: Debug for Row < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut dm = f . debug_map () ; for c in 0 .. self . stmt . column_count () { let name = self . stmt . column_name (c) . expect ("valid column index") ; dm . key (& name) ; let value = self . get_ref (c) ; match value { Ok (value) => { let dt = value . data_type () ; match value { ValueRef :: Null => { dm . value (& (dt , ())) ; } ValueRef :: Integer (i) => { dm . value (& (dt , i)) ; } ValueRef :: Real (f) => { dm . value (& (dt , f)) ; } ValueRef :: Text (s) => { dm . value (& (dt , String :: from_utf8_lossy (s))) ; } ValueRef :: Blob (b) => { dm . value (& (dt , b . len ())) ; } } } Err (ref _err) => { dm . value (& value) ; } } } dm . finish () } }
    };
}

impl_225!();