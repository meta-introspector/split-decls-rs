macro_rules! deps {
    () => {
        TomlDebugInfo!();
        Result!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl ser :: Serialize for TomlDebugInfo { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { match self { Self :: None => 0 . serialize (serializer) , Self :: LineDirectivesOnly => "line-directives-only" . serialize (serializer) , Self :: LineTablesOnly => "line-tables-only" . serialize (serializer) , Self :: Limited => 1 . serialize (serializer) , Self :: Full => 2 . serialize (serializer) , } } }
    };
}

impl_133!();