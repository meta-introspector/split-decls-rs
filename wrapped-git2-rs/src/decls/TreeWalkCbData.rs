macro_rules! deps {
    () => {
        TreeWalkCb!();
    };
}

macro_rules! TreeWalkCbData {
    () => {
        deps!();
        struct TreeWalkCbData < 'a , T > { callback : & 'a mut TreeWalkCb < 'a , T > , }
    };
}

TreeWalkCbData!()