macro_rules! deps {
    () => {
        LocalValue!();
        LocalKeyId!();
    };
}

macro_rules! LocalMap {
    () => {
        deps!();
        type LocalMap = HashMap < LocalKeyId , LocalValue > ;
    };
}

LocalMap!();