macro_rules! deps {
    () => {
        LocalKeyId!();
        LocalValue!();
    };
}

macro_rules! LocalMap {
    () => {
        deps!();
        type LocalMap = HashMap < LocalKeyId , LocalValue > ;
    };
}

LocalMap!()