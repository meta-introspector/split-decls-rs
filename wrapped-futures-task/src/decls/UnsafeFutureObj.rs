macro_rules! deps {
    () => {
        FutureObj!();
    };
}

macro_rules! UnsafeFutureObj {
    () => {
        deps!();
        # [doc = " A custom implementation of a future trait object for `FutureObj`, providing"] # [doc = " a vtable with drop support."] # [doc = ""] # [doc = " This custom representation is typically used only in `no_std` contexts,"] # [doc = " where the default `Box`-based implementation is not available."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See the safety notes on individual methods for what guarantees an"] # [doc = " implementor must provide."] pub unsafe trait UnsafeFutureObj < 'a , T > : 'a { # [doc = " Convert an owned instance into a (conceptually owned) fat pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " ## Implementor"] # [doc = ""] # [doc = " The trait implementor must guarantee that it is safe to convert the"] # [doc = " provided `*mut (dyn Future<Output = T> + 'a)` into a `Pin<&mut (dyn"] # [doc = " Future<Output = T> + 'a)>` and call methods on it, non-reentrantly,"] # [doc = " until `UnsafeFutureObj::drop` is called with it."] fn into_raw (self) -> * mut (dyn Future < Output = T > + 'a) ; # [doc = " Drops the future represented by the given fat pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " ## Implementor"] # [doc = ""] # [doc = " The trait implementor must guarantee that it is safe to call this"] # [doc = " function once per `into_raw` invocation."] # [doc = ""] # [doc = " ## Caller"] # [doc = ""] # [doc = " The caller must ensure:"] # [doc = ""] # [doc = "  * the pointer passed was obtained from an `into_raw` invocation from"] # [doc = "    this same trait object"] # [doc = "  * the pointer is not currently in use as a `Pin<&mut (dyn Future<Output"] # [doc = "    = T> + 'a)>`"] # [doc = "  * the pointer must not be used again after this function is called"] unsafe fn drop (ptr : * mut (dyn Future < Output = T > + 'a)) ; }
    };
}

UnsafeFutureObj!();