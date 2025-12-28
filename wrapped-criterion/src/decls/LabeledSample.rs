macro_rules! deps {
    () => {
        Sample!();
        Float!();
    };
}

macro_rules! LabeledSample {
    () => {
        deps!();
        # [doc = " A classified/labeled sample."] # [doc = ""] # [doc = " The labeled data can be accessed using the indexing operator. The order of the data points is"] # [doc = " retained."] # [doc = ""] # [doc = " NOTE: Due to limitations in the indexing traits, only the label is returned. Once the"] # [doc = " `IndexGet` trait lands in stdlib, the indexing operation will return a `(data_point, label)`"] # [doc = " pair."] # [derive (Clone , Copy)] pub struct LabeledSample < 'a , A > where A : Float , { fences : (A , A , A , A) , sample : & 'a Sample < A > , }
    };
}

LabeledSample!();