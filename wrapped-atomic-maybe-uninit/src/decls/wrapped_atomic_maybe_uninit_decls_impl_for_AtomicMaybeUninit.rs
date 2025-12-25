use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Primitive> AtomicMaybeUninit<T> {
    /// Creates a new atomic value from a potentially uninitialized value.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::mem::MaybeUninit;
    ///
    /// use atomic_maybe_uninit::AtomicMaybeUninit;
    ///
    /// let v = AtomicMaybeUninit::new(MaybeUninit::new(5_i32));
    ///
    /// // Equivalent to:
    /// let v = AtomicMaybeUninit::from(5_i32);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(v: MaybeUninit<T>) -> Self {
        Self {
            v: UnsafeCell::new(v),
            _align: [],
        }
    }
    const_fn! {
        const_if : #[cfg(not(atomic_maybe_uninit_no_const_mut_refs))]; #[doc =
        " Creates a new reference to an atomic value from a pointer."] #[doc = ""] #[doc
        = " This is `const fn` on Rust 1.83+."] #[doc = ""] #[doc = " # Safety"] #[doc =
        ""] #[doc =
        " * `ptr` must be aligned to `align_of::<AtomicMaybeUninit<T>>()` (note that on some platforms this"]
        #[doc = "   can be bigger than `align_of::<MaybeUninit<T>>()`)."] #[doc =
        " * `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`."]
        #[doc =
        " * Non-atomic accesses to the value behind `ptr` must have a happens-before"]
        #[doc =
        "   relationship with atomic accesses via the returned value (or vice-versa)."]
        #[doc =
        "   * In other words, time periods where the value is accessed atomically may not"]
        #[doc = "     overlap with periods where the value is accessed non-atomically."]
        #[doc =
        "   * This requirement is trivially satisfied if `ptr` is never used non-atomically"]
        #[doc =
        "     for the duration of lifetime `'a`. Most use cases should be able to follow"]
        #[doc = "     this guideline."] #[doc =
        "   * This requirement is also trivially satisfied if all accesses (atomic or not) are"]
        #[doc = "     done from the same thread."] #[doc =
        " * This method must not be used to create overlapping or mixed-size atomic"]
        #[doc = "   accesses, as these are not supported by the memory model."] #[doc =
        ""] #[doc = " [valid]: core::ptr#safety"] #[inline] #[must_use] pub const unsafe
        fn from_ptr <'a > (ptr : * mut MaybeUninit < T >) -> &'a Self { unsafe { &* ptr
        .cast::< Self > () } }
    }
    const_fn! {
        const_if : #[cfg(not(atomic_maybe_uninit_no_const_mut_refs))]; #[doc =
        " Returns a mutable reference to the underlying value."] #[doc = ""] #[doc =
        " This is safe because the mutable reference guarantees that no other threads are"]
        #[doc = " concurrently accessing the atomic data."] #[doc = ""] #[doc =
        " This is `const fn` on Rust 1.83+."] #[doc = ""] #[doc = " # Examples"] #[doc =
        ""] #[doc = " ```"] #[doc = " use std::mem::MaybeUninit;"] #[doc = ""] #[doc =
        " use atomic_maybe_uninit::AtomicMaybeUninit;"] #[doc = ""] #[doc =
        " let mut v = AtomicMaybeUninit::from(5_i32);"] #[doc =
        " unsafe { assert_eq!((*v.get_mut()).assume_init(), 5) }"] #[doc =
        " *v.get_mut() = MaybeUninit::new(10);"] #[doc =
        " unsafe { assert_eq!((*v.get_mut()).assume_init(), 10) }"] #[doc = " ```"]
        #[inline] pub const fn get_mut(& mut self) -> & mut MaybeUninit < T > { unsafe {
        & mut * self.as_ptr() } }
    }
    /// Consumes the atomic and returns the contained value.
    ///
    /// This is safe because passing `self` by value guarantees that no other threads are
    /// concurrently accessing the atomic data.
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_maybe_uninit::AtomicMaybeUninit;
    ///
    /// let v = AtomicMaybeUninit::from(5_i32);
    /// unsafe { assert_eq!(v.into_inner().assume_init(), 5) }
    /// ```
    #[inline]
    pub const fn into_inner(self) -> MaybeUninit<T> {
        unsafe { utils::transmute_copy_by_val::<Self, MaybeUninit<T>>(self) }
    }
    /// Loads a value from the atomic value.
    ///
    /// `load` takes an [`Ordering`] argument which describes the memory ordering of this operation.
    /// Possible values are [`SeqCst`], [`Acquire`] and [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `order` is [`Release`] or [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering;
    ///
    /// use atomic_maybe_uninit::AtomicMaybeUninit;
    ///
    /// let v = AtomicMaybeUninit::from(5_i32);
    /// unsafe { assert_eq!(v.load(Ordering::Relaxed).assume_init(), 5) }
    /// ```
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn load(&self, order: Ordering) -> MaybeUninit<T>
    where
        T: AtomicLoad,
    {
        utils::assert_load_ordering(order);
        unsafe { T::atomic_load(self.v.get(), order) }
    }
    /// Stores a value into the atomic value.
    ///
    /// `store` takes an [`Ordering`] argument which describes the memory ordering of this operation.
    ///  Possible values are [`SeqCst`], [`Release`] and [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `order` is [`Acquire`] or [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{mem::MaybeUninit, sync::atomic::Ordering};
    ///
    /// use atomic_maybe_uninit::AtomicMaybeUninit;
    ///
    /// let v = AtomicMaybeUninit::from(5_i32);
    /// v.store(MaybeUninit::new(10), Ordering::Relaxed);
    /// unsafe { assert_eq!(v.load(Ordering::Relaxed).assume_init(), 10) }
    /// ```
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn store(&self, val: MaybeUninit<T>, order: Ordering)
    where
        T: AtomicStore,
    {
        utils::assert_store_ordering(order);
        #[cfg(atomic_maybe_uninit_pre_llvm_20)]
        let val = core::hint::black_box(val);
        unsafe { T::atomic_store(self.v.get(), val, order) }
    }
    /// Stores a value into the atomic value, returning the previous value.
    ///
    /// `swap` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{mem::MaybeUninit, sync::atomic::Ordering};
    ///
    /// use atomic_maybe_uninit::AtomicMaybeUninit;
    ///
    /// let v = AtomicMaybeUninit::from(5_i32);
    /// unsafe {
    ///     assert_eq!(v.swap(MaybeUninit::new(10), Ordering::Relaxed).assume_init(), 5);
    ///     assert_eq!(v.load(Ordering::Relaxed).assume_init(), 10);
    /// }
    /// ```
    #[inline]
    pub fn swap(&self, val: MaybeUninit<T>, order: Ordering) -> MaybeUninit<T>
    where
        T: AtomicSwap,
    {
        #[cfg(atomic_maybe_uninit_pre_llvm_20)]
        let val = core::hint::black_box(val);
        unsafe { T::atomic_swap(self.v.get(), val, order) }
    }
    /// Stores a value into the atomic value if the current value is the same as
    /// the `current` value. Here, "the same" is determined using byte-wise
    /// equality, not `PartialEq`.
    ///
    /// The return value is a result indicating whether the new value was written and
    /// containing the previous value. On success this value is guaranteed to be equal to
    /// `current`.
    ///
    /// `compare_exchange` takes two [`Ordering`] arguments to describe the memory
    /// ordering of this operation. `success` describes the required ordering for the
    /// read-modify-write operation that takes place if the comparison with `current` succeeds.
    /// `failure` describes the required ordering for the load operation that takes place when
    /// the comparison fails. Using [`Acquire`] as success ordering makes the store part
    /// of this operation [`Relaxed`], and using [`Release`] makes the successful load
    /// [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `failure` is [`Release`], [`AcqRel`].
    ///
    /// # Notes
    ///
    /// Comparison of two values containing uninitialized bytes may fail even if
    /// they are equivalent as Rust's type, because values can be byte-wise
    /// inequal even when they are equal as Rust values.
    ///
    /// For example, the following example could be an infinite loop:
    ///
    /// ```no_run
    /// use std::{
    ///     mem::{self, MaybeUninit},
    ///     sync::atomic::Ordering,
    /// };
    ///
    /// use atomic_maybe_uninit::AtomicMaybeUninit;
    ///
    /// #[derive(Clone, Copy, PartialEq, Eq)]
    /// #[repr(C, align(4))]
    /// struct Test(u8, u16);
    ///
    /// unsafe {
    ///     let x = mem::transmute::<Test, MaybeUninit<u32>>(Test(0, 0));
    ///     let v = AtomicMaybeUninit::new(x);
    ///     while v
    ///         .compare_exchange(
    ///             mem::transmute::<Test, MaybeUninit<u32>>(Test(0, 0)),
    ///             mem::transmute::<Test, MaybeUninit<u32>>(Test(1, 0)),
    ///             Ordering::AcqRel,
    ///             Ordering::Acquire,
    ///         )
    ///         .is_err()
    ///     {}
    /// }
    /// ```
    ///
    /// To work around this problem, you need to use a helper like the following.
    ///
    /// ```
    /// # if cfg!(valgrind) { return; }
    /// # use std::{
    /// #     mem::{self, MaybeUninit},
    /// #     sync::atomic::Ordering,
    /// # };
    /// # use atomic_maybe_uninit::AtomicMaybeUninit;
    /// # #[derive(Clone, Copy, PartialEq, Eq)]
    /// # #[repr(C, align(4))]
    /// # struct Test(u8, u16);
    /// // Adapted from https://github.com/crossbeam-rs/crossbeam/blob/crossbeam-utils-0.8.10/crossbeam-utils/src/atomic/atomic_cell.rs#L1081-L1110
    /// unsafe fn atomic_compare_exchange(
    ///     v: &AtomicMaybeUninit<u32>,
    ///     mut current: Test,
    ///     new: Test,
    /// ) -> Result<Test, Test> {
    ///     let mut current_raw = mem::transmute::<Test, MaybeUninit<u32>>(current);
    ///     let new_raw = mem::transmute::<Test, MaybeUninit<u32>>(new);
    ///     loop {
    ///         match v.compare_exchange_weak(current_raw, new_raw, Ordering::AcqRel, Ordering::Acquire)
    ///         {
    ///             Ok(_) => {
    ///                 // The values are byte-wise equal; for `Test` we know this implies they are `PartialEq`-equal.
    ///                 break Ok(current);
    ///             }
    ///             Err(previous_raw) => {
    ///                 let previous = mem::transmute::<MaybeUninit<u32>, Test>(previous_raw);
    ///
    ///                 if !Test::eq(&previous, &current) {
    ///                     break Err(previous);
    ///                 }
    ///
    ///                 // The compare-exchange operation has failed and didn't store `new`. The
    ///                 // failure is either spurious, or `previous` was semantically equal to
    ///                 // `current` but not byte-equal. Let's retry with `previous` as the new
    ///                 // `current`.
    ///                 current = previous;
    ///                 current_raw = previous_raw;
    ///             }
    ///         }
    ///     }
    /// }
    ///
    /// unsafe {
    ///     let x = mem::transmute::<Test, MaybeUninit<u32>>(Test(0, 0));
    ///     let v = AtomicMaybeUninit::new(x);
    ///     while atomic_compare_exchange(&v, Test(0, 0), Test(1, 0)).is_err() {}
    /// }
    /// ```
    ///
    /// Also, Valgrind reports "Conditional jump or move depends on uninitialized value(s)"
    /// error if there is such a comparison -- which is correct, that's exactly
    /// what the implementation does, but we are doing this inside inline
    /// assembly so it should be fine. (Effectively we are adding partial
    /// `freeze` capabilities to Rust via inline assembly. This pattern has not
    /// been blessed by the language team, but is also not known to cause any
    /// problems.)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{mem::MaybeUninit, sync::atomic::Ordering};
    ///
    /// use atomic_maybe_uninit::AtomicMaybeUninit;
    ///
    /// unsafe {
    ///     let v = AtomicMaybeUninit::from(5_i32);
    ///
    ///     assert_eq!(
    ///         v.compare_exchange(
    ///             MaybeUninit::new(5),
    ///             MaybeUninit::new(10),
    ///             Ordering::Acquire,
    ///             Ordering::Relaxed
    ///         )
    ///         .unwrap()
    ///         .assume_init(),
    ///         5
    ///     );
    ///     assert_eq!(v.load(Ordering::Relaxed).assume_init(), 10);
    ///
    ///     assert_eq!(
    ///         v.compare_exchange(
    ///             MaybeUninit::new(6),
    ///             MaybeUninit::new(12),
    ///             Ordering::SeqCst,
    ///             Ordering::Acquire
    ///         )
    ///         .unwrap_err()
    ///         .assume_init(),
    ///         10
    ///     );
    ///     assert_eq!(v.load(Ordering::Relaxed).assume_init(), 10);
    /// }
    /// ```
    #[doc(alias = "compare_and_swap")]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn compare_exchange(
        &self,
        current: MaybeUninit<T>,
        new: MaybeUninit<T>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<MaybeUninit<T>, MaybeUninit<T>>
    where
        T: AtomicCompareExchange,
    {
        utils::assert_compare_exchange_ordering(success, failure);
        #[cfg(atomic_maybe_uninit_pre_llvm_20)]
        let current = core::hint::black_box(current);
        #[cfg(atomic_maybe_uninit_pre_llvm_20)]
        let new = core::hint::black_box(new);
        let (out, ok) =
            unsafe { T::atomic_compare_exchange(self.v.get(), current, new, success, failure) };
        if ok {
            Ok(out)
        } else {
            Err(out)
        }
    }
    /// Stores a value into the atomic value if the current value is the same as
    /// the `current` value. Here, "the same" is determined using byte-wise
    /// equality, not `PartialEq`.
    ///
    /// This function is allowed to spuriously fail even when the comparison succeeds,
    /// which can result in more efficient code on some platforms. The return value
    /// is a result indicating whether the new value was written and containing
    /// the previous value.
    ///
    /// `compare_exchange_weak` takes two [`Ordering`] arguments to describe the memory
    /// ordering of this operation. `success` describes the required ordering for the
    /// read-modify-write operation that takes place if the comparison with `current` succeeds.
    /// `failure` describes the required ordering for the load operation that takes place when
    /// the comparison fails. Using [`Acquire`] as success ordering makes the store part
    /// of this operation [`Relaxed`], and using [`Release`] makes the successful load
    /// [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `failure` is [`Release`], [`AcqRel`].
    ///
    /// # Notes
    ///
    /// Comparison of two values containing uninitialized bytes may fail even if
    /// they are equivalent as Rust's type, because values can be byte-wise
    /// inequal even when they are equal as Rust values.
    ///
    /// See [`compare_exchange`](Self::compare_exchange) for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{mem::MaybeUninit, sync::atomic::Ordering};
    ///
    /// use atomic_maybe_uninit::AtomicMaybeUninit;
    ///
    /// let v = AtomicMaybeUninit::from(5_i32);
    ///
    /// unsafe {
    ///     let mut old = v.load(Ordering::Relaxed);
    ///     loop {
    ///         let new = old.assume_init() * 2;
    ///         match v.compare_exchange_weak(
    ///             old,
    ///             MaybeUninit::new(new),
    ///             Ordering::SeqCst,
    ///             Ordering::Relaxed,
    ///         ) {
    ///             Ok(_) => break,
    ///             Err(x) => old = x,
    ///         }
    ///     }
    /// }
    /// ```
    #[doc(alias = "compare_and_swap")]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn compare_exchange_weak(
        &self,
        current: MaybeUninit<T>,
        new: MaybeUninit<T>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<MaybeUninit<T>, MaybeUninit<T>>
    where
        T: AtomicCompareExchange,
    {
        utils::assert_compare_exchange_ordering(success, failure);
        #[cfg(atomic_maybe_uninit_pre_llvm_20)]
        let current = core::hint::black_box(current);
        #[cfg(atomic_maybe_uninit_pre_llvm_20)]
        let new = core::hint::black_box(new);
        let (out, ok) = unsafe {
            T::atomic_compare_exchange_weak(self.v.get(), current, new, success, failure)
        };
        if ok {
            Ok(out)
        } else {
            Err(out)
        }
    }
    /// Fetches the value, and applies a function to it that returns an optional
    /// new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else
    /// `Err(previous_value)`.
    ///
    /// Note: This may call the function multiple times if the value has been changed from other threads in
    /// the meantime, as long as the function returns `Some(_)`, but the function will have been applied
    /// only once to the stored value.
    ///
    /// `fetch_update` takes two [`Ordering`] arguments to describe the memory ordering of this operation.
    /// The first describes the required ordering for when the operation finally succeeds while the second
    /// describes the required ordering for loads. These correspond to the success and failure orderings of
    /// [`compare_exchange`](Self::compare_exchange) respectively.
    ///
    /// Using [`Acquire`] as success ordering makes the store part
    /// of this operation [`Relaxed`], and using [`Release`] makes the final successful load
    /// [`Relaxed`]. The (failed) load ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `fetch_order` is [`Release`], [`AcqRel`].
    ///
    /// # Considerations
    ///
    /// This method is not magic; it is not provided by the hardware.
    /// It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),
    /// and suffers from the same drawbacks.
    /// In particular, this method will not circumvent the [ABA Problem].
    ///
    /// [ABA Problem]: https://en.wikipedia.org/wiki/ABA_problem
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{mem::MaybeUninit, sync::atomic::Ordering};
    ///
    /// use atomic_maybe_uninit::AtomicMaybeUninit;
    ///
    /// unsafe {
    ///     let v = AtomicMaybeUninit::from(5_i32);
    ///     assert_eq!(
    ///         v.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None).unwrap_err().assume_init(),
    ///         5
    ///     );
    ///     assert_eq!(
    ///         v.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(MaybeUninit::new(
    ///             x.assume_init() + 1
    ///         )))
    ///         .unwrap()
    ///         .assume_init(),
    ///         5
    ///     );
    ///     assert_eq!(v.load(Ordering::SeqCst).assume_init(), 6);
    /// }
    /// ```
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        mut f: F,
    ) -> Result<MaybeUninit<T>, MaybeUninit<T>>
    where
        F: FnMut(MaybeUninit<T>) -> Option<MaybeUninit<T>>,
        T: AtomicCompareExchange,
    {
        let mut prev = self.load(fetch_order);
        while let Some(next) = f(prev) {
            match self.compare_exchange_weak(prev, next, set_order, fetch_order) {
                x @ Ok(_) => return x,
                Err(next_prev) => prev = next_prev,
            }
        }
        Err(prev)
    }
    /// Returns a mutable pointer to the underlying value.
    ///
    /// Returning an `*mut` pointer from a shared reference to this atomic is safe because the
    /// atomic types work with interior mutability. All modifications of an atomic change the value
    /// through a shared reference, and can do so safely as long as they use atomic operations. Any
    /// use of the returned raw pointer requires an `unsafe` block and still has to uphold the same
    /// restriction: operations on it must be atomic.
    #[inline]
    pub const fn as_ptr(&self) -> *mut MaybeUninit<T> {
        self.v.get()
    }
}
