macro_rules! Error {
    () => {
        # [allow (clippy :: enum_variant_names)] # [derive (Debug)] pub enum Error { AccessError { path : PathBuf , inner : io :: Error , } , CopyError { from : PathBuf , to : PathBuf , inner : io :: Error , } , SerdeError { path : PathBuf , inner : SerdeError , } , # [cfg (feature = "csv_output")] # [doc = " This API requires the following crate features to be activated: `csv_output`"] CsvError (CsvError) , }
    };
}

Error!()