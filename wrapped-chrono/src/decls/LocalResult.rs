macro_rules! deps {
    () => {
        MappedLocalTime!();
    };
}

macro_rules! LocalResult {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Debug , Copy , Eq , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] # [doc = " Old name of [`MappedLocalTime`]. See that type for more documentation."] pub enum LocalResult < T > { # [doc = " The local time maps to a single unique result."] Single (T) , # [doc = " The local time is _ambiguous_ because there is a _fold_ in the local time."] # [doc = ""] # [doc = " This variant contains the two possible results, in the order `(earliest, latest)`."] Ambiguous (T , T) , # [doc = " The local time does not exist because there is a _gap_ in the local time."] # [doc = ""] # [doc = " This variant may also be returned if there was an error while resolving the local time,"] # [doc = " caused by for example missing time zone data files, an error in an OS API, or overflow."] None , }
    };
}

LocalResult!()