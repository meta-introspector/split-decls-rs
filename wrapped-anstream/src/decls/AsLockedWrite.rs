macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! AsLockedWrite {
    () => {
        deps!();
        # [doc = " Lock a stream"] pub trait AsLockedWrite : private :: Sealed { # [doc = " Locked writer type"] type Write < 'w > : RawStream + 'w where Self : 'w ; # [doc = " Lock a stream"] fn as_locked_write (& mut self) -> Self :: Write < '_ > ; }
    };
}

AsLockedWrite!()