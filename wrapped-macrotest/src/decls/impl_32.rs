macro_rules! deps {
    () => {
        Dependency!();
        Result!();
        Error!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Dependency { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct DependencyVisitor ; impl < 'de > Visitor < 'de > for DependencyVisitor { type Value = Dependency ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a version string like \"0.9.8\" or a \
                     dependency like { version = \"0.9.8\" }" ,) } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (Dependency { version : Some (s . to_owned ()) , path : None , default_features : true , features : Vec :: new () , workspace : false , rest : Map :: new () , }) } fn visit_map < M > (self , map : M) -> Result < Self :: Value , M :: Error > where M : de :: MapAccess < 'de > , { Dependency :: deserialize (MapAccessDeserializer :: new (map)) } } deserializer . deserialize_any (DependencyVisitor) } }
    };
}

impl_32!();