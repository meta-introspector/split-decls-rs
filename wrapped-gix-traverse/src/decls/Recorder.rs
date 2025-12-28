macro_rules! deps {
    () => {
        Location!();
        Entry!();
        Visit!();
    };
}

macro_rules! Recorder {
    () => {
        deps!();
        # [doc = " A [Visit] implementation to record every observed change and keep track of the changed paths."] # [doc = ""] # [doc = " Recorders can also be instructed to track the filename only, or no location at all."] # [derive (Clone , Debug)] pub struct Recorder { path_deque : VecDeque < BString > , path : BString , # [doc = " How to track the location."] location : Option < recorder :: Location > , # [doc = " The observed entries."] pub records : Vec < recorder :: Entry > , }
    };
}

Recorder!();