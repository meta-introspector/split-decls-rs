macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ErrorRecord {
    () => {
        deps!();
        # [doc = " A path that encountered an IO error."] # [derive (Debug)] pub struct ErrorRecord { # [doc = " the path that encountered the error."] pub path : BString , # [doc = " The error"] pub error : Box < dyn std :: error :: Error + Send + Sync + 'static > , }
    };
}

ErrorRecord!();