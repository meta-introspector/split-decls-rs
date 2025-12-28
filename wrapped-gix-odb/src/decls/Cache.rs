macro_rules! deps {
    () => {
        NewObjectCacheFn!();
        NewPackCacheFn!();
        PackCache!();
        ObjectCache!();
    };
}

macro_rules! Cache {
    () => {
        deps!();
        # [doc = " A way to access objects along with pre-configured thread-local caches for packed base objects as well as objects themselves."] # [doc = ""] # [doc = " By default, no cache will be used."] pub struct Cache < S > { # [doc = " The inner provider of trait implementations we use in conjunction with our caches."] # [doc = ""] # [doc = " For calling methods on `inner`, prefer to make use of auto-dereferencing, i.e. `cache.inner_method()` instead of `cache.inner.inner_method()`."] inner : S , new_pack_cache : Option < Arc < cache :: NewPackCacheFn > > , new_object_cache : Option < Arc < cache :: NewObjectCacheFn > > , pack_cache : Option < RefCell < Box < cache :: PackCache > > > , object_cache : Option < RefCell < Box < cache :: ObjectCache > > > , }
    };
}

Cache!();