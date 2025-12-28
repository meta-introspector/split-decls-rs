macro_rules! deps {
    () => {
        Clone!();
        Note!();
        Repository!();
    };
}

macro_rules! Reference {
    () => {
        deps!();
        # [doc = " A reference that points to an object or reference, with access to its source repository."] # [doc = ""] # [doc = " Note that these are snapshots and won't recognize if they are stale."] # [derive (Clone)] pub struct Reference < 'r > { # [doc = " The actual reference data"] pub inner : gix_ref :: Reference , # [doc = " The owning repository."] pub repo : & 'r Repository , }
    };
}

Reference!();