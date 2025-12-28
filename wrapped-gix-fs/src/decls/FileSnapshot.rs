macro_rules! FileSnapshot {
    () => {
        # [doc = " A structure holding enough information to reload a value if its on-disk representation changes as determined by its modified time."] # [derive (Debug)] pub struct FileSnapshot < T : std :: fmt :: Debug > { value : T , modified : std :: time :: SystemTime , }
    };
}

FileSnapshot!();