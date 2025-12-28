macro_rules! deps {
    () => {
        Status!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [doc = " Access"] impl Status { # [doc = " Note that this is assumed true even if no new status is set, hence we assume that upon error, the caller will not continue"] # [doc = " interacting with the process."] pub fn is_success (& self) -> bool { match self { Status :: Previous => true , Status :: Unset => false , Status :: Named (n) => n == "success" , } } # [doc = " Returns true if this is an `abort` status."] pub fn is_abort (& self) -> bool { self . message () == Some ("abort") } # [doc = " Return true if the status is explicitly set to indicated delayed output processing"] pub fn is_delayed (& self) -> bool { match self { Status :: Previous | Status :: Unset => false , Status :: Named (n) => n == "delayed" , } } # [doc = " Return the status message if present."] pub fn message (& self) -> Option < & str > { match self { Status :: Previous | Status :: Unset => None , Status :: Named (msg) => msg . as_str () . into () , } } }
    };
}

impl_65!()