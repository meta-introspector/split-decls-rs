macro_rules! deps {
    () => {
        TomlTrimPathsValue!();
        TomlTrimPaths!();
        Result!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for TomlTrimPaths { fn deserialize < D > (d : D) -> Result < TomlTrimPaths , D :: Error > where D : de :: Deserializer < 'de > , { use serde :: de :: Error as _ ; let expecting = r#"a boolean, "none", "diagnostics", "macro", "object", "all", or an array with these options"# ; UntaggedEnumVisitor :: new () . expecting (expecting) . bool (| value | { Ok (if value { TomlTrimPaths :: All } else { TomlTrimPaths :: none () }) }) . string (| v | match v { "none" => Ok (TomlTrimPaths :: none ()) , "all" => Ok (TomlTrimPaths :: All) , v => { let d = v . into_deserializer () ; let err = | _ : D :: Error | { serde_untagged :: de :: Error :: custom (format ! ("expected {expecting}")) } ; TomlTrimPathsValue :: deserialize (d) . map_err (err) . map (| v | v . into ()) } }) . seq (| seq | { let seq : Vec < String > = seq . deserialize () ? ; let seq : Vec < _ > = seq . into_iter () . map (| s | TomlTrimPathsValue :: deserialize (s . into_deserializer ())) . collect :: < Result < _ , _ > > () ? ; Ok (seq . into ()) }) . deserialize (d) } }
    };
}

impl_137!();