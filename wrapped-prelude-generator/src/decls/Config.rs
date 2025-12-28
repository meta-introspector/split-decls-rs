macro_rules! Config {
    () => {
        # [derive (Debug , Deserialize , Clone)] pub struct Config { pub bins : Option < HashMap < String , String > > , # [serde (rename = "generatedOutputDir")] pub generated_output_dir : Option < PathBuf > , }
    };
}

Config!()