macro_rules! deps {
    () => {
        Filter!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Filter { # [doc = " Returns the maximum `LevelFilter` that this filter instance is"] # [doc = " configured to output."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use log::LevelFilter;"] # [doc = " use env_filter::Builder;"] # [doc = ""] # [doc = " let mut builder = Builder::new();"] # [doc = " builder.filter(Some(\"module1\"), LevelFilter::Info);"] # [doc = " builder.filter(Some(\"module2\"), LevelFilter::Error);"] # [doc = ""] # [doc = " let filter = builder.build();"] # [doc = " assert_eq!(filter.filter(), LevelFilter::Info);"] # [doc = " ```"] pub fn filter (& self) -> LevelFilter { self . directives . iter () . map (| d | d . level) . max () . unwrap_or (LevelFilter :: Off) } # [doc = " Checks if this record matches the configured filter."] pub fn matches (& self , record : & Record < '_ >) -> bool { if ! self . enabled (record . metadata ()) { return false ; } if let Some (filter) = self . filter . as_ref () { if ! filter . is_match (& record . args () . to_string ()) { return false ; } } true } # [doc = " Determines if a log message with the specified metadata would be logged."] pub fn enabled (& self , metadata : & Metadata < '_ >) -> bool { let level = metadata . level () ; let target = metadata . target () ; enabled (& self . directives , level , target) } }
    };
}

impl_8!()