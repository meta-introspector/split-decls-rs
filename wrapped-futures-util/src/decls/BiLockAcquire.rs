macro_rules! deps {
    () => {
        BiLock!();
    };
}

macro_rules! BiLockAcquire {
    () => {
        deps!();
        # [doc = " Future returned by `BiLock::lock` which will resolve when the lock is"] # [doc = " acquired."] # [cfg (feature = "bilock")] # [cfg_attr (docsrs , doc (cfg (feature = "bilock")))] # [must_use = "futures do nothing unless you `.await` or poll them"] # [derive (Debug)] pub struct BiLockAcquire < 'a , T > { bilock : & 'a BiLock < T > , }
    };
}

BiLockAcquire!()