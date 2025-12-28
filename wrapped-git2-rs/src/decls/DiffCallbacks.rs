macro_rules! deps {
    () => {
        HunkCb!();
        LineCb!();
        FileCb!();
        BinaryCb!();
    };
}

macro_rules! DiffCallbacks {
    () => {
        deps!();
        pub struct DiffCallbacks < 'a , 'b , 'c , 'd , 'e , 'f , 'g , 'h > { pub file : Option < & 'a mut FileCb < 'b > > , pub binary : Option < & 'c mut BinaryCb < 'd > > , pub hunk : Option < & 'e mut HunkCb < 'f > > , pub line : Option < & 'g mut LineCb < 'h > > , }
    };
}

DiffCallbacks!();