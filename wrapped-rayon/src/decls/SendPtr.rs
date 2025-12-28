macro_rules! SendPtr {
    () => {
        # [doc = " We need to transmit raw pointers across threads. It is possible to do this"] # [doc = " without any unsafe code by converting pointers to usize or to AtomicPtr<T>"] # [doc = " then back to a raw pointer for use. We prefer this approach because code"] # [doc = " that uses this type is more explicit."] # [doc = ""] # [doc = " Unsafe code is still required to dereference the pointer, so this type is"] # [doc = " not unsound on its own, although it does partly lift the unconditional"] # [doc = " !Send and !Sync on raw pointers. As always, dereference with care."] struct SendPtr < T > (* mut T) ;
    };
}

SendPtr!();