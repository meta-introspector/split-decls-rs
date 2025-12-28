macro_rules! RwLock {
    () => {
        # [doc = " A reader-writer lock"] # [doc = ""] # [doc = " This type of lock allows a number of readers or at most one writer at any"] # [doc = " point in time. The write portion of this lock typically allows modification"] # [doc = " of the underlying data (exclusive access) and the read portion of this lock"] # [doc = " typically allows for read-only access (shared access)."] # [doc = ""] # [doc = " The type parameter `T` represents the data that this lock protects. It is"] # [doc = " required that `T` satisfies `Send` to be shared across threads and `Sync` to"] # [doc = " allow concurrent access through readers. The RAII guards returned from the"] # [doc = " locking methods implement `Deref` (and `DerefMut` for the `write` methods)"] # [doc = " to allow access to the contained of the lock."] pub struct RwLock < R , T : ? Sized > { raw : R , data : UnsafeCell < T > , }
    };
}

RwLock!()