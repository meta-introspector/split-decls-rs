macro_rules! deps {
    () => {
        Database!();
        ObjectIdentifier!();
    };
}

macro_rules! Names {
    () => {
        deps!();
        # [doc = " Iterator returning the multiple names that may be associated with an OID."] pub struct Names < 'a > { database : Database < 'a > , oid : ObjectIdentifier , position : usize , }
    };
}

Names!()