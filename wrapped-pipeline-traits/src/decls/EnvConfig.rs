macro_rules! EnvConfig {
    () => {
        # [derive (Debug , Deserialize , Clone)] pub struct EnvConfig { # [serde (rename = "HOME")] pub home : Option < String > , # [serde (rename = "CARGO_HOME")] pub cargo_home : Option < String > , }
    };
}

EnvConfig!()