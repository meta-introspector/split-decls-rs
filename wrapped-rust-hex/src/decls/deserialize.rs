macro_rules! deps {
    () => {
        FromHex!();
    };
}

macro_rules! deserialize {
    () => {
        deps!();
        # [doc = " Deserializes a hex string into raw bytes."] # [doc = ""] # [doc = " Both, upper and lower case characters are valid in the input string and can"] # [doc = " even be mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings)."] pub fn deserialize < 'de , D , T > (deserializer : D) -> Result < T , D :: Error > where D : Deserializer < 'de > , T : FromHex , < T as FromHex > :: Error : fmt :: Display , { struct HexStrVisitor < T > (PhantomData < T >) ; impl < 'de , T > Visitor < 'de > for HexStrVisitor < T > where T : FromHex , < T as FromHex > :: Error : fmt :: Display , { type Value = T ; fn expecting (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "a hex encoded string") } fn visit_str < E > (self , data : & str) -> Result < Self :: Value , E > where E : Error , { FromHex :: from_hex (data) . map_err (Error :: custom) } fn visit_borrowed_str < E > (self , data : & 'de str) -> Result < Self :: Value , E > where E : Error , { FromHex :: from_hex (data) . map_err (Error :: custom) } } deserializer . deserialize_str (HexStrVisitor (PhantomData)) }
    };
}

deserialize!();