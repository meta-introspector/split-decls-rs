macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! str_newtype {
    () => {
        deps!();
        macro_rules ! str_newtype { ($ (# [doc = $ docs : literal]) * $ name : ident) => { $ (# [doc = $ docs]) * # [derive (Serialize , Debug , Clone , Eq , PartialOrd , Ord , Hash)] # [serde (transparent)] pub struct $ name < T : AsRef < str > = String > (T) ; impl < T : AsRef < str >> $ name < T > { # [doc = " Convert the wrapped string into its inner type `T`"] pub fn into_inner (self) -> T { self . 0 } } impl < T : AsRef < str >> AsRef < str > for $ name < T > { fn as_ref (& self) -> & str { self . 0 . as_ref () } } impl < T : AsRef < str >> std :: ops :: Deref for $ name < T > { type Target = T ; fn deref (& self) -> & Self :: Target { & self . 0 } } impl < T : AsRef < str >> std :: borrow :: Borrow < str > for $ name < T > { fn borrow (& self) -> & str { self . 0 . as_ref () } } impl <'a > std :: str :: FromStr for $ name < String > { type Err = std :: convert :: Infallible ; fn from_str (value : & str) -> Result < Self , Self :: Err > { Ok (Self :: new (value . to_owned ())) } } impl <'de , T : AsRef < str > + serde :: Deserialize <'de >> serde :: Deserialize <'de > for $ name < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer <'de >, { let inner = T :: deserialize (deserializer) ?; Ok (Self :: new (inner)) } } impl < T : AsRef < str >> fmt :: Display for $ name < T > { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { self . 0 . as_ref () . fmt (f) } } impl < T : AsRef < str >> $ name < T > { # [doc = " Create a new wrapped string"] pub fn new (name : T) -> Self { Self (name) } } impl < T : AsRef < str >, Rhs : AsRef < str >> PartialEq < Rhs > for $ name < T > { fn eq (& self , other : & Rhs) -> bool { self . as_ref () == other . as_ref () } } } ; }
    };
}

str_newtype!()