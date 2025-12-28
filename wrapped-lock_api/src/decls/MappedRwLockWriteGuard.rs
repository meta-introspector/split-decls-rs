macro_rules! deps {
    () => {
        RwLock!();
        RawRwLock!();
        RwLockWriteGuard!();
    };
}

macro_rules! MappedRwLockWriteGuard {
    () => {
        deps!();
        # [doc = " An RAII write lock guard returned by `RwLockWriteGuard::map`, which can point to a"] # [doc = " subfield of the protected data."] # [doc = ""] # [doc = " The main difference between `MappedRwLockWriteGuard` and `RwLockWriteGuard` is that the"] # [doc = " former doesn't support temporarily unlocking and re-locking, since that"] # [doc = " could introduce soundness issues if the locked object is modified by another"] # [doc = " thread."] # [clippy :: has_significant_drop] # [must_use = "if unused the RwLock will immediately unlock"] pub struct MappedRwLockWriteGuard < 'a , R : RawRwLock , T : ? Sized > { raw : & 'a R , data : * mut T , marker : PhantomData < & 'a mut T > , }
    };
}

MappedRwLockWriteGuard!()