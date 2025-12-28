macro_rules! deps {
    () => {
        RawRwLock!();
        MappedRwLockWriteGuard!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + Send + 'a > Send for MappedRwLockWriteGuard < 'a , R , T > where R :: GuardMarker : Send { }
    };
}

impl_194!()