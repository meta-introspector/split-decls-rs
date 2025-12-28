macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { format : Default :: default () , tree_prefix : None , modification_time : std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . map (| t | t . as_secs () as i64) . unwrap_or_default () , } } }
    };
}

impl_3!();