macro_rules! deps {
    () => {
        Signature!();
        SignatureRef!();
    };
}

macro_rules! convert {
    () => {
        deps!();
        mod convert { use gix_date :: parse :: TimeBuf ; use crate :: { Signature , SignatureRef } ; impl Signature { # [doc = " Borrow this instance as immutable, serializing the `time` field into `buf`."] # [doc = ""] # [doc = " Commonly used as [`signature.to_ref(&mut TimeBuf::default())`](TimeBuf::default)."] pub fn to_ref < 'a > (& 'a self , time_buf : & 'a mut TimeBuf) -> SignatureRef < 'a > { SignatureRef { name : self . name . as_ref () , email : self . email . as_ref () , time : self . time . to_str (time_buf) , } } } # [doc = " Note that this conversion is lossy due to the lenient parsing of the [`time`](SignatureRef::time) field."] impl From < SignatureRef < '_ > > for Signature { fn from (other : SignatureRef < '_ >) -> Signature { Signature { name : other . name . to_owned () , email : other . email . to_owned () , time : other . time () . unwrap_or_default () , } } } }
    };
}

convert!()