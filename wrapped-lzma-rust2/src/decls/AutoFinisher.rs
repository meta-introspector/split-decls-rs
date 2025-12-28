macro_rules! deps {
    () => {
        AutoFinish!();
    };
}

macro_rules! AutoFinisher {
    () => {
        deps!();
        # [doc = " A wrapper around a writer that finishes the stream on drop."] # [allow (private_bounds)] pub struct AutoFinisher < T : AutoFinish > (Option < T >) ;
    };
}

AutoFinisher!()