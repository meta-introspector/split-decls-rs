// Generated macro for str_newtype (macro)
macro_rules! Depcrate_manifeststr_newtype {
() => {
// Module: crate::manifest
// Provides: {"str_newtype"}
// Dependencies: {}
macro_rules ! str_newtype { ($ name : ident) => { # [doc = " Verified string newtype"] # [derive (Serialize , Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [serde (transparent)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct $ name < T : AsRef < str > = String > (T) ; impl < T : AsRef < str >> $ name < T > { pub fn into_inner (self) -> T { self . 0 } } impl < T : AsRef < str >> AsRef < str > for $ name < T > { fn as_ref (& self) -> & str { self . 0 . as_ref () } } impl < T : AsRef < str >> std :: ops :: Deref for $ name < T > { type Target = T ; fn deref (& self) -> & Self :: Target { & self . 0 } } impl < T : AsRef < str >> std :: borrow :: Borrow < str > for $ name < T > { fn borrow (& self) -> & str { self . 0 . as_ref () } } impl <'a > std :: str :: FromStr for $ name < String > { type Err = restricted_names :: NameValidationError ; fn from_str (value : & str) -> Result < Self , Self :: Err > { Self :: new (value . to_owned ()) } } impl <'de , T : AsRef < str > + serde :: Deserialize <'de >> serde :: Deserialize <'de > for $ name < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer <'de >, { let inner = T :: deserialize (deserializer) ?; Self :: new (inner) . map_err (serde :: de :: Error :: custom) } } impl < T : AsRef < str >> Display for $ name < T > { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { self . 0 . as_ref () . fmt (f) } } } ; }
};
}
