macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! RemoteName {
    () => {
        deps!();
        # [doc = " A key that represents a remote name, either as url or symbolic name."] pub type RemoteName = Any < validate :: RemoteName > ;
    };
}

RemoteName!();