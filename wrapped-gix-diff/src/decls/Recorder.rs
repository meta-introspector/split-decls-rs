macro_rules! deps {
    () => {
        Location!();
        Visit!();
        Change!();
    };
}

macro_rules! Recorder {
    () => {
        deps!();
        # [doc = " A [Visit] implementation to record every observed change and keep track of the changed paths."] # [derive (Clone , Debug)] pub struct Recorder { path_deque : VecDeque < BString > , path : BString , location : Option < recorder :: Location > , # [doc = " The observed changes."] pub records : Vec < recorder :: Change > , }
    };
}

Recorder!();