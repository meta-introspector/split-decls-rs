macro_rules! deps {
    () => {
        BStr!();
    };
}

macro_rules! bstr_serde {
    () => {
        deps!();
        # [cfg (feature = "serde")] mod bstr_serde { use core :: fmt ; use serde :: { de :: Error , de :: Visitor , Deserialize , Deserializer , Serialize , Serializer , } ; use crate :: bstr :: BStr ; impl Serialize for BStr { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self . as_bytes ()) } } impl < 'a , 'de : 'a > Deserialize < 'de > for & 'a BStr { # [inline] fn deserialize < D > (deserializer : D) -> Result < & 'a BStr , D :: Error > where D : Deserializer < 'de > , { struct BStrVisitor ; impl < 'de > Visitor < 'de > for BStrVisitor { type Value = & 'de BStr ; fn expecting (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("a borrowed byte string") } # [inline] fn visit_borrowed_bytes < E : Error > (self , value : & 'de [u8] ,) -> Result < & 'de BStr , E > { Ok (BStr :: new (value)) } # [inline] fn visit_borrowed_str < E : Error > (self , value : & 'de str ,) -> Result < & 'de BStr , E > { Ok (BStr :: new (value)) } } deserializer . deserialize_bytes (BStrVisitor) } } }
    };
}

bstr_serde!();