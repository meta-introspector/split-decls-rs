macro_rules! deps {
    () => {
        Error!();
        StreamingBuffer!();
        Result!();
    };
}

macro_rules! impl_1027 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < W > StreamingBuffer < W > { # [doc = " Create a new `StreamingBuffer` backed by the given writer."] pub fn new (writer : W) -> Self { StreamingBuffer { writer , len : 0 , result : Ok (()) , } } # [doc = " Unwraps this [`StreamingBuffer`] giving back the original writer."] pub fn into_inner (self) -> W { self . writer } # [doc = " Returns any error that occurred during writing."] pub fn result (& mut self) -> Result < () , io :: Error > { mem :: replace (& mut self . result , Ok (())) } }
    };
}

impl_1027!()