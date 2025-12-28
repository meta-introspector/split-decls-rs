macro_rules! deps {
    () => {
        Entry!();
        DirEntry!();
    };
}

macro_rules! AccelerateLookup {
    () => {
        deps!();
        # [doc = " A backing store for accelerating lookups of entries in a case-sensitive and case-insensitive manner."] pub struct AccelerateLookup < 'a > { # [doc = " The entries themselves, hashed by their full icase path."] # [doc = " Icase-clashes are handled in order of occurrence and are all available for iteration."] icase_entries : hashbrown :: HashTable < & 'a Entry > , # [doc = " Each hash in this table corresponds to a directory containing one or more entries."] icase_dirs : hashbrown :: HashTable < DirEntry < 'a > > , }
    };
}

AccelerateLookup!()