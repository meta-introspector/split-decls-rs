macro_rules! deps {
    () => {
        AtomicLoad!();
        AtomicStore!();
    };
}

macro_rules! AtomicSwap {
    () => {
        deps!();
        # [doc = " Atomic swap."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside of `atomic-maybe-uninit`."] # [cfg_attr (not (atomic_maybe_uninit_no_diagnostic_namespace) , diagnostic :: on_unimplemented (message = "atomic swap of `{Self}` is not available on this target" , label = "this associated function is not available on this target" , note = "see <https://docs.rs/atomic-maybe-uninit/latest/atomic_maybe_uninit/#platform-support> for more."))] pub trait AtomicSwap : AtomicLoad + AtomicStore { # [doc = " Stores a value into `dst`, returning the previous value."] # [doc = ""] # [doc = " `atomic_swap` takes an [`Ordering`] argument which describes the memory ordering"] # [doc = " of this operation. All ordering modes are possible. Note that using"] # [doc = " [`Acquire`] makes the store part of this operation [`Relaxed`], and"] # [doc = " using [`Release`] makes the load part [`Relaxed`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Behavior is undefined if any of the following conditions are violated:"] # [doc = ""] # [doc = " - `dst` must be valid for both reads and writes."] # [doc = " - `dst` must be properly aligned **to the size of `Self`**."] # [doc = "   (For example, if `Self` is `u128`, `dst` must be aligned to 16-byte even if the alignment of `u128` is 8-byte.)"] # [doc = " - `order` must be [`SeqCst`], [`AcqRel`], [`Acquire`], [`Release`], or [`Relaxed`]."] # [doc = ""] # [doc = " The rules for the validity of the pointer follow [the rules applied to"] # [doc = " functions exposed by the standard library's `ptr` module][validity],"] # [doc = " except that concurrent atomic operations on `dst` are allowed if the"] # [doc = " pointer go through [`UnsafeCell::get`]."] # [doc = ""] # [doc = " [validity]: core::ptr#safety"] unsafe fn atomic_swap (dst : * mut MaybeUninit < Self > , val : MaybeUninit < Self > , order : Ordering ,) -> MaybeUninit < Self > ; }
    };
}

AtomicSwap!()