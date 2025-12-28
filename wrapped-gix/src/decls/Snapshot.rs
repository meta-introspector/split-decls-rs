macro_rules! deps {
    () => {
        Repository!();
        Note!();
    };
}

macro_rules! Snapshot {
    () => {
        deps!();
        # [doc = " A platform to access configuration values as read from disk."] # [doc = ""] # [doc = " Note that these values won't update even if the underlying file(s) change."] pub struct Snapshot < 'repo > { # [doc = " The owning repository."] pub repo : & 'repo Repository , }
    };
}

Snapshot!();