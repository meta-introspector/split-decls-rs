macro_rules! deps {
    () => {
        BiLock!();
    };
}

macro_rules! ReuniteError {
    () => {
        deps!();
        # [doc = " Error indicating two `BiLock<T>`s were not two halves of a whole, and"] # [doc = " thus could not be `reunite`d."] # [cfg_attr (docsrs , doc (cfg (feature = "bilock")))] pub struct ReuniteError < T > (pub BiLock < T > , pub BiLock < T >) ;
    };
}

ReuniteError!()