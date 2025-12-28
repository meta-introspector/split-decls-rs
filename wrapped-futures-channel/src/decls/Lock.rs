macro_rules! Lock {
    () => {
        # [doc = " A \"mutex\" around a value, similar to `std::sync::Mutex<T>`."] # [doc = ""] # [doc = " This lock only supports the `try_lock` operation, however, and does not"] # [doc = " implement poisoning."] # [derive (Debug)] pub (crate) struct Lock < T > { locked : AtomicBool , data : UnsafeCell < T > , }
    };
}

Lock!()