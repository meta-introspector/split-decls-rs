macro_rules! AtomicMaybeUninit {
    () => {
        # [doc = " A potentially uninitialized integer type which can be safely shared between threads."] # [doc = ""] # [doc = " This type has the same in-memory representation as the underlying"] # [doc = " value type, `MaybeUninit<T>`."] # [repr (C)] pub struct AtomicMaybeUninit < T : Primitive > { v : UnsafeCell < MaybeUninit < T > > , # [doc = " `[T::Align; 0]` ensures alignment is at least that of `T::Align`."] # [doc = ""] # [doc = " This is needed because x86's u64 is 4-byte aligned and x86_64's u128 is"] # [doc = " 8-byte aligned and atomic operations normally require alignment greater"] # [doc = " than or equal to the size."] _align : [T :: Align ; 0] , }
    };
}

AtomicMaybeUninit!()