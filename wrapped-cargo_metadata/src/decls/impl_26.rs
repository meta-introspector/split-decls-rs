macro_rules! deps {
    () => {
        ArtifactDebuginfo!();
        Result!();
        Error!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for ArtifactDebuginfo { fn deserialize < D > (d : D) -> Result < ArtifactDebuginfo , D :: Error > where D : de :: Deserializer < 'de > , { struct Visitor ; impl de :: Visitor < '_ > for Visitor { type Value = ArtifactDebuginfo ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("an integer or string") } fn visit_i64 < E > (self , value : i64) -> Result < ArtifactDebuginfo , E > where E : de :: Error , { let debuginfo = match value { 0 => ArtifactDebuginfo :: None , 1 => ArtifactDebuginfo :: Limited , 2 => ArtifactDebuginfo :: Full , n => ArtifactDebuginfo :: UnknownInt (n) , } ; Ok (debuginfo) } fn visit_u64 < E > (self , value : u64) -> Result < ArtifactDebuginfo , E > where E : de :: Error , { self . visit_i64 (value as i64) } fn visit_str < E > (self , value : & str) -> Result < ArtifactDebuginfo , E > where E : de :: Error , { let debuginfo = match value { "none" => ArtifactDebuginfo :: None , "limited" => ArtifactDebuginfo :: Limited , "full" => ArtifactDebuginfo :: Full , "line-directives-only" => ArtifactDebuginfo :: LineDirectivesOnly , "line-tables-only" => ArtifactDebuginfo :: LineTablesOnly , s => ArtifactDebuginfo :: UnknownString (s . to_string ()) , } ; Ok (debuginfo) } fn visit_unit < E > (self) -> Result < ArtifactDebuginfo , E > where E : de :: Error , { Ok (ArtifactDebuginfo :: None) } } d . deserialize_any (Visitor) } }
    };
}

impl_26!();