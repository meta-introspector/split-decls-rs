macro_rules! deps {
    () => {
        Result!();
        TomlInheritedField!();
        VecStringOrBool!();
        InheritableField!();
        InheritableVecStringOrBool!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for InheritableVecStringOrBool { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct Visitor ; impl < 'de > de :: Visitor < 'de > for Visitor { type Value = InheritableVecStringOrBool ; fn expecting (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("a boolean, a vector of strings, or workspace") } fn visit_bool < E > (self , v : bool) -> Result < Self :: Value , E > where E : de :: Error , { let b = de :: value :: BoolDeserializer :: new (v) ; VecStringOrBool :: deserialize (b) . map (InheritableField :: Value) } fn visit_seq < A > (self , v : A) -> Result < Self :: Value , A :: Error > where A : de :: SeqAccess < 'de > , { let seq = de :: value :: SeqAccessDeserializer :: new (v) ; VecStringOrBool :: deserialize (seq) . map (InheritableField :: Value) } fn visit_map < V > (self , map : V) -> Result < Self :: Value , V :: Error > where V : de :: MapAccess < 'de > , { let mvd = de :: value :: MapAccessDeserializer :: new (map) ; TomlInheritedField :: deserialize (mvd) . map (InheritableField :: Inherit) } } d . deserialize_any (Visitor) } }
    };
}

impl_100!()