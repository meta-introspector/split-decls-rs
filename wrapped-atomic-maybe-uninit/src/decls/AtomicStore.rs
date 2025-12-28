macro_rules! deps {
    () => {
        Primitive!();
    };
}

macro_rules! AtomicStore {
    () => {
        deps!();
        # [doc = " Atomic store."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside of `atomic-maybe-uninit`."] # [cfg_attr (not (atomic_maybe_uninit_no_diagnostic_namespace) , diagnostic :: on_unimplemented (message = "atomic store of `{Self}` is not available on this target" , label = "this associated function is not available on this target" , note = "see <https://docs.rs/atomic-maybe-uninit/latest/atomic_maybe_uninit/#platform-support> for more."))] pub trait AtomicStore : Primitive { # [doc = " Stores a value into `dst`."] # [doc = ""] # [doc = " `atomic_store` takes an [`Ordering`] argument which describes the memory ordering of this operation."] # [doc = "  Possible values are [`SeqCst`], [`Release`] and [`Relaxed`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Behavior is undefined if any of the following conditions are violated:"] # [doc = ""] # [doc = " - `dst` must be valid for writes"] # [doc = " - `dst` must be properly aligned **to the size of `Self`**."] # [doc = "   (For example, if `Self` is `u128`, `dst` must be aligned to 16-byte even if the alignment of `u128` is 8-byte.)"] # [doc = " - `order` must be [`SeqCst`], [`Release`], or [`Relaxed`]."] # [doc = ""] # [doc = " The rules for the validity of the pointer follow [the rules applied to"] # [doc = " functions exposed by the standard library's `ptr` module][validity],"] # [doc = " except that concurrent atomic operations on `dst` are allowed if the"] # [doc = " pointer go through [`UnsafeCell::get`]."] # [doc = ""] # [doc = " [validity]: core::ptr#safety"] unsafe fn atomic_store (dst : * mut MaybeUninit < Self > , val : MaybeUninit < Self > , order : Ordering) ; }
    };
}

AtomicStore!()