macro_rules! Location {
    () => {
        # [doc = " Describe how to track the location of an entry."] # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Location { # [doc = " Track the entire path, relative to the repository."] Path , # [doc = " Keep only the file-name as location, which may be enough for some calculations."] # [doc = ""] # [doc = " This is less expensive than tracking the entire `Path`."] FileName , }
    };
}

Location!()