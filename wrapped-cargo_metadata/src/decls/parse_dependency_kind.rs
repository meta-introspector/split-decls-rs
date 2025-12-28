macro_rules! deps {
    () => {
        Error!();
        DependencyKind!();
        Result!();
    };
}

macro_rules! parse_dependency_kind {
    () => {
        deps!();
        # [doc = " The `kind` can be `null`, which is interpreted as the default - `Normal`."] pub (super) fn parse_dependency_kind < 'de , D > (d : D) -> Result < DependencyKind , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (d) . map (| x : Option < _ > | x . unwrap_or_default ()) }
    };
}

parse_dependency_kind!()