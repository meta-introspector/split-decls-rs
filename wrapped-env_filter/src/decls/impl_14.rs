macro_rules! deps {
    () => {
        FilteredLog!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T : Log > Log for FilteredLog < T > { # [doc = " Determines if a log message with the specified metadata would be logged."] # [doc = ""] # [doc = " For the wrapped log, this returns `true` only if both the filter and the wrapped log return `true`."] fn enabled (& self , metadata : & log :: Metadata < '_ >) -> bool { self . filter . enabled (metadata) && self . log . enabled (metadata) } # [doc = " Logs the record."] # [doc = ""] # [doc = " Forwards the record to the wrapped log, but only if the record matches the filter."] fn log (& self , record : & log :: Record < '_ >) { if self . filter . matches (record) { self . log . log (record) ; } } # [doc = " Flushes any buffered records."] # [doc = ""] # [doc = " Forwards directly to the wrapped log."] fn flush (& self) { self . log . flush () ; } }
    };
}

impl_14!()