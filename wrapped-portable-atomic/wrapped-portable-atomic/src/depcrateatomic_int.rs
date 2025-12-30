// Generated macro for atomic_int (macro)
macro_rules! Depcrateatomic_int {
() => {
// Module: crate
// Provides: {"atomic_int"}
// Dependencies: {}
macro_rules ! atomic_int { ($ atomic_type : ident , $ int_type : ident , $ align : literal , $ cfg_has_atomic_cas_or_amo32_or_8 : ident , $ cfg_no_atomic_cas_or_amo32_or_8 : ident $ (, # [$ cfg_float : meta] $ atomic_float_type : ident , $ float_type : ident) ?) => { doc_comment ! { concat ! ("An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`" , stringify ! ($ int_type) , "`].

If the compiler and the platform support atomic loads and stores of [`" , stringify ! ($ int_type) , "`], this type is a wrapper for the standard library's `" , stringify ! ($ atomic_type) , "`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`" , stringify ! ($ atomic_type) , "::is_lock_free()`] to check whether
atomic instructions or locks will be used.
") , # [repr (C , align ($ align))] pub struct $ atomic_type { inner : imp ::$ atomic_type , } } impl Default for $ atomic_type { # [inline] fn default () -> Self { Self :: new ($ int_type :: default ()) } } impl From <$ int_type > for $ atomic_type { # [inline] fn from (v : $ int_type) -> Self { Self :: new (v) } } # [cfg (not (portable_atomic_no_core_unwind_safe))] impl core :: panic :: RefUnwindSafe for $ atomic_type { } # [cfg (all (portable_atomic_no_core_unwind_safe , feature = "std"))] impl std :: panic :: RefUnwindSafe for $ atomic_type { } impl_debug_and_serde ! ($ atomic_type) ; impl $ atomic_type { doc_comment ! { concat ! ("Creates a new atomic integer.

# Examples

```
use portable_atomic::" , stringify ! ($ atomic_type) , ";

let atomic_forty_two = " , stringify ! ($ atomic_type) , "::new(42);
```") , # [inline] # [must_use] pub const fn new (v : $ int_type) -> Self { static_assert_layout ! ($ atomic_type , $ int_type) ; Self { inner : imp ::$ atomic_type :: new (v) } } } # [cfg (not (portable_atomic_no_const_mut_refs))] doc_comment ! { concat ! ("Creates a new reference to an atomic integer from a pointer.

This is `const fn` on Rust 1.83+.

# Safety

* `ptr` must be aligned to `align_of::<" , stringify ! ($ atomic_type) , ">()` (note that on some platforms this
  can be bigger than `align_of::<" , stringify ! ($ int_type) , ">()`).
* `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
* If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
  behind `ptr` must have a happens-before relationship with atomic accesses via
  the returned value (or vice-versa).
  * In other words, time periods where the value is accessed atomically may not
    overlap with periods where the value is accessed non-atomically.
  * This requirement is trivially satisfied if `ptr` is never used non-atomically
    for the duration of lifetime `'a`. Most use cases should be able to follow
    this guideline.
  * This requirement is also trivially satisfied if all accesses (atomic or not) are
    done from the same thread.
* If this atomic type is *not* lock-free:
  * Any accesses to the value behind `ptr` must have a happens-before relationship
    with accesses via the returned value (or vice-versa).
  * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
    be compatible with operations performed by this atomic type.
* This method must not be used to create overlapping or mixed-size atomic
  accesses, as these are not supported by the memory model.

[valid]: core::ptr#safety") , # [inline] # [must_use] pub const unsafe fn from_ptr <'a > (ptr : * mut $ int_type) -> &'a Self { # [allow (clippy :: cast_ptr_alignment)] unsafe { &* (ptr as * mut Self) } } } # [cfg (portable_atomic_no_const_mut_refs)] doc_comment ! { concat ! ("Creates a new reference to an atomic integer from a pointer.

This is `const fn` on Rust 1.83+.

# Safety

* `ptr` must be aligned to `align_of::<" , stringify ! ($ atomic_type) , ">()` (note that on some platforms this
  can be bigger than `align_of::<" , stringify ! ($ int_type) , ">()`).
* `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
* If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
  behind `ptr` must have a happens-before relationship with atomic accesses via
  the returned value (or vice-versa).
  * In other words, time periods where the value is accessed atomically may not
    overlap with periods where the value is accessed non-atomically.
  * This requirement is trivially satisfied if `ptr` is never used non-atomically
    for the duration of lifetime `'a`. Most use cases should be able to follow
    this guideline.
  * This requirement is also trivially satisfied if all accesses (atomic or not) are
    done from the same thread.
* If this atomic type is *not* lock-free:
  * Any accesses to the value behind `ptr` must have a happens-before relationship
    with accesses via the returned value (or vice-versa).
  * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
    be compatible with operations performed by this atomic type.
* This method must not be used to create overlapping or mixed-size atomic
  accesses, as these are not supported by the memory model.

[valid]: core::ptr#safety") , # [inline] # [must_use] pub unsafe fn from_ptr <'a > (ptr : * mut $ int_type) -> &'a Self { # [allow (clippy :: cast_ptr_alignment)] unsafe { &* (ptr as * mut Self) } } } doc_comment ! { concat ! ("Returns `true` if operations on values of this type are lock-free.

If the compiler or the platform doesn't support the necessary
atomic instructions, global locks for every potentially
concurrent atomic operation will be used.

# Examples

```
use portable_atomic::" , stringify ! ($ atomic_type) , ";

let is_lock_free = " , stringify ! ($ atomic_type) , "::is_lock_free();
```") , # [inline] # [must_use] pub fn is_lock_free () -> bool { < imp ::$ atomic_type >:: is_lock_free () } } doc_comment ! { concat ! ("Returns `true` if operations on values of this type are lock-free.

If the compiler or the platform doesn't support the necessary
atomic instructions, global locks for every potentially
concurrent atomic operation will be used.

**Note:** If the atomic operation relies on dynamic CPU feature detection,
this type may be lock-free even if the function returns false.

# Examples

```
use portable_atomic::" , stringify ! ($ atomic_type) , ";

const IS_ALWAYS_LOCK_FREE: bool = " , stringify ! ($ atomic_type) , "::is_always_lock_free();
```") , # [inline] # [must_use] pub const fn is_always_lock_free () -> bool { < imp ::$ atomic_type >:: IS_ALWAYS_LOCK_FREE } } # [cfg (test)] # [cfg_attr (all (valgrind , target_arch = "powerpc64") , allow (dead_code))] const IS_ALWAYS_LOCK_FREE : bool = Self :: is_always_lock_free () ; # [cfg (not (portable_atomic_no_const_mut_refs))] doc_comment ! { concat ! ("Returns a mutable reference to the underlying integer.\n
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.

This is `const fn` on Rust 1.83+.

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let mut some_var = " , stringify ! ($ atomic_type) , "::new(10);
assert_eq!(*some_var.get_mut(), 10);
*some_var.get_mut() = 5;
assert_eq!(some_var.load(Ordering::SeqCst), 5);
```") , # [inline] pub const fn get_mut (& mut self) -> & mut $ int_type { unsafe { & mut * self . as_ptr () } } } # [cfg (portable_atomic_no_const_mut_refs)] doc_comment ! { concat ! ("Returns a mutable reference to the underlying integer.\n
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.

This is `const fn` on Rust 1.83+.

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let mut some_var = " , stringify ! ($ atomic_type) , "::new(10);
assert_eq!(*some_var.get_mut(), 10);
*some_var.get_mut() = 5;
assert_eq!(some_var.load(Ordering::SeqCst), 5);
```") , # [inline] pub fn get_mut (& mut self) -> & mut $ int_type { unsafe { & mut * self . as_ptr () } } } # [cfg (not (portable_atomic_no_const_transmute))] doc_comment ! { concat ! ("Consumes the atomic and returns the contained value.

This is safe because passing `self` by value guarantees that no other threads are
concurrently accessing the atomic data.

This is `const fn` on Rust 1.56+.

# Examples

```
use portable_atomic::" , stringify ! ($ atomic_type) , ";

let some_var = " , stringify ! ($ atomic_type) , "::new(5);
assert_eq!(some_var.into_inner(), 5);
```") , # [inline] pub const fn into_inner (self) -> $ int_type { unsafe { core :: mem :: transmute (self) } } } # [cfg (portable_atomic_no_const_transmute)] doc_comment ! { concat ! ("Consumes the atomic and returns the contained value.

This is safe because passing `self` by value guarantees that no other threads are
concurrently accessing the atomic data.

This is `const fn` on Rust 1.56+.

# Examples

```
use portable_atomic::" , stringify ! ($ atomic_type) , ";

let some_var = " , stringify ! ($ atomic_type) , "::new(5);
assert_eq!(some_var.into_inner(), 5);
```") , # [inline] pub fn into_inner (self) -> $ int_type { unsafe { core :: mem :: transmute (self) } } } doc_comment ! { concat ! ("Loads a value from the atomic integer.

`load` takes an [`Ordering`] argument which describes the memory ordering of this operation.
Possible values are [`SeqCst`], [`Acquire`] and [`Relaxed`].

# Panics

Panics if `order` is [`Release`] or [`AcqRel`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let some_var = " , stringify ! ($ atomic_type) , "::new(5);

assert_eq!(some_var.load(Ordering::Relaxed), 5);
```") , # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn load (& self , order : Ordering) -> $ int_type { self . inner . load (order) } } doc_comment ! { concat ! ("Stores a value into the atomic integer.

`store` takes an [`Ordering`] argument which describes the memory ordering of this operation.
Possible values are [`SeqCst`], [`Release`] and [`Relaxed`].

# Panics

Panics if `order` is [`Acquire`] or [`AcqRel`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let some_var = " , stringify ! ($ atomic_type) , "::new(5);

some_var.store(10, Ordering::Relaxed);
assert_eq!(some_var.load(Ordering::Relaxed), 10);
```") , # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn store (& self , val : $ int_type , order : Ordering) { self . inner . store (val , order) } } cfg_has_atomic_cas_or_amo32 ! { $ cfg_has_atomic_cas_or_amo32_or_8 ! { doc_comment ! { concat ! ("Stores a value into the atomic integer, returning the previous value.

`swap` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let some_var = " , stringify ! ($ atomic_type) , "::new(5);

assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn swap (& self , val : $ int_type , order : Ordering) -> $ int_type { self . inner . swap (val , order) } } } cfg_has_atomic_cas ! { doc_comment ! { concat ! ("Stores a value into the atomic integer if the current value is the same as
the `current` value.

The return value is a result indicating whether the new value was written and
containing the previous value. On success this value is guaranteed to be equal to
`current`.

`compare_exchange` takes two [`Ordering`] arguments to describe the memory
ordering of this operation. `success` describes the required ordering for the
read-modify-write operation that takes place if the comparison with `current` succeeds.
`failure` describes the required ordering for the load operation that takes place when
the comparison fails. Using [`Acquire`] as success ordering makes the store part
of this operation [`Relaxed`], and using [`Release`] makes the successful load
[`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].

# Panics

Panics if `failure` is [`Release`], [`AcqRel`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let some_var = " , stringify ! ($ atomic_type) , "::new(5);

assert_eq!(
    some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),
    Ok(5),
);
assert_eq!(some_var.load(Ordering::Relaxed), 10);

assert_eq!(
    some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),
    Err(10),
);
assert_eq!(some_var.load(Ordering::Relaxed), 10);
```") , # [cfg_attr (docsrs , doc (alias = "compare_and_swap"))] # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn compare_exchange (& self , current : $ int_type , new : $ int_type , success : Ordering , failure : Ordering ,) -> Result <$ int_type , $ int_type > { self . inner . compare_exchange (current , new , success , failure) } } doc_comment ! { concat ! ("Stores a value into the atomic integer if the current value is the same as
the `current` value.
Unlike [`compare_exchange`](Self::compare_exchange)
this function is allowed to spuriously fail even
when the comparison succeeds, which can result in more efficient code on some
platforms. The return value is a result indicating whether the new value was
written and containing the previous value.

`compare_exchange_weak` takes two [`Ordering`] arguments to describe the memory
ordering of this operation. `success` describes the required ordering for the
read-modify-write operation that takes place if the comparison with `current` succeeds.
`failure` describes the required ordering for the load operation that takes place when
the comparison fails. Using [`Acquire`] as success ordering makes the store part
of this operation [`Relaxed`], and using [`Release`] makes the successful load
[`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].

# Panics

Panics if `failure` is [`Release`], [`AcqRel`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let val = " , stringify ! ($ atomic_type) , "::new(4);

let mut old = val.load(Ordering::Relaxed);
loop {
    let new = old * 2;
    match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
        Ok(_) => break,
        Err(x) => old = x,
    }
}
```") , # [cfg_attr (docsrs , doc (alias = "compare_and_swap"))] # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn compare_exchange_weak (& self , current : $ int_type , new : $ int_type , success : Ordering , failure : Ordering ,) -> Result <$ int_type , $ int_type > { self . inner . compare_exchange_weak (current , new , success , failure) } } } $ cfg_has_atomic_cas_or_amo32_or_8 ! { doc_comment ! { concat ! ("Adds to the current value, returning the previous value.

This operation wraps around on overflow.

`fetch_add` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0);
assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_add (& self , val : $ int_type , order : Ordering) -> $ int_type { self . inner . fetch_add (val , order) } } doc_comment ! { concat ! ("Adds to the current value.

This operation wraps around on overflow.

Unlike `fetch_add`, this does not return the previous value.

`add` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_add` on some platforms.

- MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0);
foo.add(10, Ordering::SeqCst);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn add (& self , val : $ int_type , order : Ordering) { self . inner . add (val , order) ; } } doc_comment ! { concat ! ("Subtracts from the current value, returning the previous value.

This operation wraps around on overflow.

`fetch_sub` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(20);
assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_sub (& self , val : $ int_type , order : Ordering) -> $ int_type { self . inner . fetch_sub (val , order) } } doc_comment ! { concat ! ("Subtracts from the current value.

This operation wraps around on overflow.

Unlike `fetch_sub`, this does not return the previous value.

`sub` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_sub` on some platforms.

- MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(20);
foo.sub(10, Ordering::SeqCst);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn sub (& self , val : $ int_type , order : Ordering) { self . inner . sub (val , order) ; } } } doc_comment ! { concat ! ("Bitwise \"and\" with the current value.

Performs a bitwise \"and\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_and` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0b101101);
assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b100001);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_and (& self , val : $ int_type , order : Ordering) -> $ int_type { self . inner . fetch_and (val , order) } } doc_comment ! { concat ! ("Bitwise \"and\" with the current value.

Performs a bitwise \"and\" operation on the current value and the argument `val`, and
sets the new value to the result.

Unlike `fetch_and`, this does not return the previous value.

`and` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_and` on some platforms.

- x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)
- MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

Note: On x86/x86_64, the use of either function should not usually
affect the generated code, because LLVM can properly optimize the case
where the result is unused.

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0b101101);
assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b100001);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn and (& self , val : $ int_type , order : Ordering) { self . inner . and (val , order) ; } } cfg_has_atomic_cas ! { doc_comment ! { concat ! ("Bitwise \"nand\" with the current value.

Performs a bitwise \"nand\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_nand` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0x13);
assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);
assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_nand (& self , val : $ int_type , order : Ordering) -> $ int_type { self . inner . fetch_nand (val , order) } } } doc_comment ! { concat ! ("Bitwise \"or\" with the current value.

Performs a bitwise \"or\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_or` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0b101101);
assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b111111);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_or (& self , val : $ int_type , order : Ordering) -> $ int_type { self . inner . fetch_or (val , order) } } doc_comment ! { concat ! ("Bitwise \"or\" with the current value.

Performs a bitwise \"or\" operation on the current value and the argument `val`, and
sets the new value to the result.

Unlike `fetch_or`, this does not return the previous value.

`or` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_or` on some platforms.

- x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)
- MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

Note: On x86/x86_64, the use of either function should not usually
affect the generated code, because LLVM can properly optimize the case
where the result is unused.

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0b101101);
assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b111111);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn or (& self , val : $ int_type , order : Ordering) { self . inner . or (val , order) ; } } doc_comment ! { concat ! ("Bitwise \"xor\" with the current value.

Performs a bitwise \"xor\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_xor` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0b101101);
assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b011110);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_xor (& self , val : $ int_type , order : Ordering) -> $ int_type { self . inner . fetch_xor (val , order) } } doc_comment ! { concat ! ("Bitwise \"xor\" with the current value.

Performs a bitwise \"xor\" operation on the current value and the argument `val`, and
sets the new value to the result.

Unlike `fetch_xor`, this does not return the previous value.

`xor` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_xor` on some platforms.

- x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)
- MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

Note: On x86/x86_64, the use of either function should not usually
affect the generated code, because LLVM can properly optimize the case
where the result is unused.

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0b101101);
foo.xor(0b110011, Ordering::SeqCst);
assert_eq!(foo.load(Ordering::SeqCst), 0b011110);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn xor (& self , val : $ int_type , order : Ordering) { self . inner . xor (val , order) ; } } cfg_has_atomic_cas ! { doc_comment ! { concat ! ("Fetches the value, and applies a function to it that returns an optional
new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else
`Err(previous_value)`.

Note: This may call the function multiple times if the value has been changed from other threads in
the meantime, as long as the function returns `Some(_)`, but the function will have been applied
only once to the stored value.

`fetch_update` takes two [`Ordering`] arguments to describe the memory ordering of this operation.
The first describes the required ordering for when the operation finally succeeds while the second
describes the required ordering for loads. These correspond to the success and failure orderings of
[`compare_exchange`](Self::compare_exchange) respectively.

Using [`Acquire`] as success ordering makes the store part
of this operation [`Relaxed`], and using [`Release`] makes the final successful load
[`Relaxed`]. The (failed) load ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].

# Panics

Panics if `fetch_order` is [`Release`], [`AcqRel`].

# Considerations

This method is not magic; it is not provided by the hardware.
It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),
and suffers from the same drawbacks.
In particular, this method will not circumvent the [ABA Problem].

[ABA Problem]: https://en.wikipedia.org/wiki/ABA_problem

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let x = " , stringify ! ($ atomic_type) , "::new(7);
assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));
assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));
assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));
assert_eq!(x.load(Ordering::SeqCst), 9);
```") , # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn fetch_update < F > (& self , set_order : Ordering , fetch_order : Ordering , mut f : F ,) -> Result <$ int_type , $ int_type > where F : FnMut ($ int_type) -> Option <$ int_type >, { let mut prev = self . load (fetch_order) ; while let Some (next) = f (prev) { match self . compare_exchange_weak (prev , next , set_order , fetch_order) { x @ Ok (_) => return x , Err (next_prev) => prev = next_prev , } } Err (prev) } } } $ cfg_has_atomic_cas_or_amo32_or_8 ! { doc_comment ! { concat ! ("Maximum with the current value.

Finds the maximum of the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_max` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(23);
assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);
assert_eq!(foo.load(Ordering::SeqCst), 42);
```

If you want to obtain the maximum value in one step, you can use the following:

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(23);
let bar = 42;
let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);
assert!(max_foo == 42);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_max (& self , val : $ int_type , order : Ordering) -> $ int_type { self . inner . fetch_max (val , order) } } doc_comment ! { concat ! ("Minimum with the current value.

Finds the minimum of the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_min` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(23);
assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);
assert_eq!(foo.load(Ordering::Relaxed), 23);
assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);
assert_eq!(foo.load(Ordering::Relaxed), 22);
```

If you want to obtain the minimum value in one step, you can use the following:

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(23);
let bar = 12;
let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);
assert_eq!(min_foo, 12);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_min (& self , val : $ int_type , order : Ordering) -> $ int_type { self . inner . fetch_min (val , order) } } } doc_comment ! { concat ! ("Sets the bit at the specified bit-position to 1.

Returns `true` if the specified bit was previously set to 1.

`bit_set` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0b0000);
assert!(!foo.bit_set(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0001);
assert!(foo.bit_set(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0001);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn bit_set (& self , bit : u32 , order : Ordering) -> bool { self . inner . bit_set (bit , order) } } doc_comment ! { concat ! ("Clears the bit at the specified bit-position to 1.

Returns `true` if the specified bit was previously set to 1.

`bit_clear` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0b0001);
assert!(foo.bit_clear(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0000);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn bit_clear (& self , bit : u32 , order : Ordering) -> bool { self . inner . bit_clear (bit , order) } } doc_comment ! { concat ! ("Toggles the bit at the specified bit-position.

Returns `true` if the specified bit was previously set to 1.

`bit_toggle` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0b0000);
assert!(!foo.bit_toggle(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0001);
assert!(foo.bit_toggle(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0000);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn bit_toggle (& self , bit : u32 , order : Ordering) -> bool { self . inner . bit_toggle (bit , order) } } doc_comment ! { concat ! ("Logical negates the current value, and sets the new value to the result.

Returns the previous value.

`fetch_not` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0);
assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);
assert_eq!(foo.load(Ordering::Relaxed), !0);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_not (& self , order : Ordering) -> $ int_type { self . inner . fetch_not (order) } } doc_comment ! { concat ! ("Logical negates the current value, and sets the new value to the result.

Unlike `fetch_not`, this does not return the previous value.

`not` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_not` on some platforms.

- x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)
- MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(0);
foo.not(Ordering::Relaxed);
assert_eq!(foo.load(Ordering::Relaxed), !0);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn not (& self , order : Ordering) { self . inner . not (order) ; } } cfg_has_atomic_cas ! { doc_comment ! { concat ! ("Negates the current value, and sets the new value to the result.

Returns the previous value.

`fetch_neg` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(5);
assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);
assert_eq!(foo.load(Ordering::Relaxed), 5_" , stringify ! ($ int_type) , ".wrapping_neg());
assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_" , stringify ! ($ int_type) , ".wrapping_neg());
assert_eq!(foo.load(Ordering::Relaxed), 5);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_neg (& self , order : Ordering) -> $ int_type { self . inner . fetch_neg (order) } } doc_comment ! { concat ! ("Negates the current value, and sets the new value to the result.

Unlike `fetch_neg`, this does not return the previous value.

`neg` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_neg` on some platforms.

- x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

# Examples

```
use portable_atomic::{" , stringify ! ($ atomic_type) , ", Ordering};

let foo = " , stringify ! ($ atomic_type) , "::new(5);
foo.neg(Ordering::Relaxed);
assert_eq!(foo.load(Ordering::Relaxed), 5_" , stringify ! ($ int_type) , ".wrapping_neg());
foo.neg(Ordering::Relaxed);
assert_eq!(foo.load(Ordering::Relaxed), 5);
```") , # [inline] # [cfg_attr (miri , track_caller)] pub fn neg (& self , order : Ordering) { self . inner . neg (order) ; } } } } const_fn ! { const_if : # [cfg (not (portable_atomic_no_const_raw_ptr_deref))] ; # [doc = " Returns a mutable pointer to the underlying integer."] # [doc = ""] # [doc = " Returning an `*mut` pointer from a shared reference to this atomic is"] # [doc = " safe because the atomic types work with interior mutability. Any use of"] # [doc = " the returned raw pointer requires an `unsafe` block and has to uphold"] # [doc = " the safety requirements. If there is concurrent access, note the following"] # [doc = " additional safety requirements:"] # [doc = ""] # [doc = " - If this atomic type is [lock-free](Self::is_lock_free), any concurrent"] # [doc = "   operations on it must be atomic."] # [doc = " - Otherwise, any concurrent operations on it must be compatible with"] # [doc = "   operations performed by this atomic type."] # [doc = ""] # [doc = " This is `const fn` on Rust 1.58+."] # [inline] pub const fn as_ptr (& self) -> * mut $ int_type { self . inner . as_ptr () } } } # [cfg (not (feature = "require-cas"))] cfg_no_atomic_cas ! { # [doc (hidden)] # [allow (unused_variables , clippy :: unused_self , clippy :: extra_unused_lifetimes)] impl <'a > $ atomic_type { $ cfg_no_atomic_cas_or_amo32_or_8 ! { # [inline] pub fn swap (& self , val : $ int_type , order : Ordering) -> $ int_type where &'a Self : HasSwap , { unimplemented ! () } } # [inline] pub fn compare_exchange (& self , current : $ int_type , new : $ int_type , success : Ordering , failure : Ordering ,) -> Result <$ int_type , $ int_type > where &'a Self : HasCompareExchange , { unimplemented ! () } # [inline] pub fn compare_exchange_weak (& self , current : $ int_type , new : $ int_type , success : Ordering , failure : Ordering ,) -> Result <$ int_type , $ int_type > where &'a Self : HasCompareExchangeWeak , { unimplemented ! () } $ cfg_no_atomic_cas_or_amo32_or_8 ! { # [inline] pub fn fetch_add (& self , val : $ int_type , order : Ordering) -> $ int_type where &'a Self : HasFetchAdd , { unimplemented ! () } # [inline] pub fn add (& self , val : $ int_type , order : Ordering) where &'a Self : HasAdd , { unimplemented ! () } # [inline] pub fn fetch_sub (& self , val : $ int_type , order : Ordering) -> $ int_type where &'a Self : HasFetchSub , { unimplemented ! () } # [inline] pub fn sub (& self , val : $ int_type , order : Ordering) where &'a Self : HasSub , { unimplemented ! () } } cfg_no_atomic_cas_or_amo32 ! { # [inline] pub fn fetch_and (& self , val : $ int_type , order : Ordering) -> $ int_type where &'a Self : HasFetchAnd , { unimplemented ! () } # [inline] pub fn and (& self , val : $ int_type , order : Ordering) where &'a Self : HasAnd , { unimplemented ! () } } # [inline] pub fn fetch_nand (& self , val : $ int_type , order : Ordering) -> $ int_type where &'a Self : HasFetchNand , { unimplemented ! () } cfg_no_atomic_cas_or_amo32 ! { # [inline] pub fn fetch_or (& self , val : $ int_type , order : Ordering) -> $ int_type where &'a Self : HasFetchOr , { unimplemented ! () } # [inline] pub fn or (& self , val : $ int_type , order : Ordering) where &'a Self : HasOr , { unimplemented ! () } # [inline] pub fn fetch_xor (& self , val : $ int_type , order : Ordering) -> $ int_type where &'a Self : HasFetchXor , { unimplemented ! () } # [inline] pub fn xor (& self , val : $ int_type , order : Ordering) where &'a Self : HasXor , { unimplemented ! () } } # [inline] pub fn fetch_update < F > (& self , set_order : Ordering , fetch_order : Ordering , f : F ,) -> Result <$ int_type , $ int_type > where F : FnMut ($ int_type) -> Option <$ int_type >, &'a Self : HasFetchUpdate , { unimplemented ! () } $ cfg_no_atomic_cas_or_amo32_or_8 ! { # [inline] pub fn fetch_max (& self , val : $ int_type , order : Ordering) -> $ int_type where &'a Self : HasFetchMax , { unimplemented ! () } # [inline] pub fn fetch_min (& self , val : $ int_type , order : Ordering) -> $ int_type where &'a Self : HasFetchMin , { unimplemented ! () } } cfg_no_atomic_cas_or_amo32 ! { # [inline] pub fn bit_set (& self , bit : u32 , order : Ordering) -> bool where &'a Self : HasBitSet , { unimplemented ! () } # [inline] pub fn bit_clear (& self , bit : u32 , order : Ordering) -> bool where &'a Self : HasBitClear , { unimplemented ! () } # [inline] pub fn bit_toggle (& self , bit : u32 , order : Ordering) -> bool where &'a Self : HasBitToggle , { unimplemented ! () } # [inline] pub fn fetch_not (& self , order : Ordering) -> $ int_type where &'a Self : HasFetchNot , { unimplemented ! () } # [inline] pub fn not (& self , order : Ordering) where &'a Self : HasNot , { unimplemented ! () } } # [inline] pub fn fetch_neg (& self , order : Ordering) -> $ int_type where &'a Self : HasFetchNeg , { unimplemented ! () } # [inline] pub fn neg (& self , order : Ordering) where &'a Self : HasNeg , { unimplemented ! () } } } $ (# [$ cfg_float] atomic_int ! (float , # [$ cfg_float] $ atomic_float_type , $ float_type , $ atomic_type , $ int_type , $ align) ;) ? } ; (float , # [$ cfg_float : meta] $ atomic_type : ident , $ float_type : ident , $ atomic_int_type : ident , $ int_type : ident , $ align : literal) => { doc_comment ! { concat ! ("A floating point type which can be safely shared between threads.

This type has the same in-memory representation as the underlying floating point type,
[`" , stringify ! ($ float_type) , "`].
") , # [cfg_attr (docsrs , doc ($ cfg_float))] # [repr (C , align ($ align))] pub struct $ atomic_type { inner : imp :: float ::$ atomic_type , } } impl Default for $ atomic_type { # [inline] fn default () -> Self { Self :: new ($ float_type :: default ()) } } impl From <$ float_type > for $ atomic_type { # [inline] fn from (v : $ float_type) -> Self { Self :: new (v) } } # [cfg (not (portable_atomic_no_core_unwind_safe))] impl core :: panic :: RefUnwindSafe for $ atomic_type { } # [cfg (all (portable_atomic_no_core_unwind_safe , feature = "std"))] impl std :: panic :: RefUnwindSafe for $ atomic_type { } impl_debug_and_serde ! ($ atomic_type) ; impl $ atomic_type { # [doc = " Creates a new atomic float."] # [inline] # [must_use] pub const fn new (v : $ float_type) -> Self { static_assert_layout ! ($ atomic_type , $ float_type) ; Self { inner : imp :: float ::$ atomic_type :: new (v) } } # [cfg (not (portable_atomic_no_const_mut_refs))] doc_comment ! { concat ! ("Creates a new reference to an atomic float from a pointer.

This is `const fn` on Rust 1.83+.

# Safety

* `ptr` must be aligned to `align_of::<" , stringify ! ($ atomic_type) , ">()` (note that on some platforms this
  can be bigger than `align_of::<" , stringify ! ($ float_type) , ">()`).
* `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
* If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
  behind `ptr` must have a happens-before relationship with atomic accesses via
  the returned value (or vice-versa).
  * In other words, time periods where the value is accessed atomically may not
    overlap with periods where the value is accessed non-atomically.
  * This requirement is trivially satisfied if `ptr` is never used non-atomically
    for the duration of lifetime `'a`. Most use cases should be able to follow
    this guideline.
  * This requirement is also trivially satisfied if all accesses (atomic or not) are
    done from the same thread.
* If this atomic type is *not* lock-free:
  * Any accesses to the value behind `ptr` must have a happens-before relationship
    with accesses via the returned value (or vice-versa).
  * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
    be compatible with operations performed by this atomic type.
* This method must not be used to create overlapping or mixed-size atomic
  accesses, as these are not supported by the memory model.

[valid]: core::ptr#safety") , # [inline] # [must_use] pub const unsafe fn from_ptr <'a > (ptr : * mut $ float_type) -> &'a Self { # [allow (clippy :: cast_ptr_alignment)] unsafe { &* (ptr as * mut Self) } } } # [cfg (portable_atomic_no_const_mut_refs)] doc_comment ! { concat ! ("Creates a new reference to an atomic float from a pointer.

This is `const fn` on Rust 1.83+.

# Safety

* `ptr` must be aligned to `align_of::<" , stringify ! ($ atomic_type) , ">()` (note that on some platforms this
  can be bigger than `align_of::<" , stringify ! ($ float_type) , ">()`).
* `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
* If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
  behind `ptr` must have a happens-before relationship with atomic accesses via
  the returned value (or vice-versa).
  * In other words, time periods where the value is accessed atomically may not
    overlap with periods where the value is accessed non-atomically.
  * This requirement is trivially satisfied if `ptr` is never used non-atomically
    for the duration of lifetime `'a`. Most use cases should be able to follow
    this guideline.
  * This requirement is also trivially satisfied if all accesses (atomic or not) are
    done from the same thread.
* If this atomic type is *not* lock-free:
  * Any accesses to the value behind `ptr` must have a happens-before relationship
    with accesses via the returned value (or vice-versa).
  * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
    be compatible with operations performed by this atomic type.
* This method must not be used to create overlapping or mixed-size atomic
  accesses, as these are not supported by the memory model.

[valid]: core::ptr#safety") , # [inline] # [must_use] pub unsafe fn from_ptr <'a > (ptr : * mut $ float_type) -> &'a Self { # [allow (clippy :: cast_ptr_alignment)] unsafe { &* (ptr as * mut Self) } } } # [doc = " Returns `true` if operations on values of this type are lock-free."] # [doc = ""] # [doc = " If the compiler or the platform doesn't support the necessary"] # [doc = " atomic instructions, global locks for every potentially"] # [doc = " concurrent atomic operation will be used."] # [inline] # [must_use] pub fn is_lock_free () -> bool { < imp :: float ::$ atomic_type >:: is_lock_free () } # [doc = " Returns `true` if operations on values of this type are lock-free."] # [doc = ""] # [doc = " If the compiler or the platform doesn't support the necessary"] # [doc = " atomic instructions, global locks for every potentially"] # [doc = " concurrent atomic operation will be used."] # [doc = ""] # [doc = " **Note:** If the atomic operation relies on dynamic CPU feature detection,"] # [doc = " this type may be lock-free even if the function returns false."] # [inline] # [must_use] pub const fn is_always_lock_free () -> bool { < imp :: float ::$ atomic_type >:: IS_ALWAYS_LOCK_FREE } # [cfg (test)] const IS_ALWAYS_LOCK_FREE : bool = Self :: is_always_lock_free () ; const_fn ! { const_if : # [cfg (not (portable_atomic_no_const_mut_refs))] ; # [doc = " Returns a mutable reference to the underlying float."] # [doc = ""] # [doc = " This is safe because the mutable reference guarantees that no other threads are"] # [doc = " concurrently accessing the atomic data."] # [doc = ""] # [doc = " This is `const fn` on Rust 1.83+."] # [inline] pub const fn get_mut (& mut self) -> & mut $ float_type { unsafe { & mut * self . as_ptr () } } } const_fn ! { const_if : # [cfg (not (portable_atomic_no_const_transmute))] ; # [doc = " Consumes the atomic and returns the contained value."] # [doc = ""] # [doc = " This is safe because passing `self` by value guarantees that no other threads are"] # [doc = " concurrently accessing the atomic data."] # [doc = ""] # [doc = " This is `const fn` on Rust 1.56+."] # [inline] pub const fn into_inner (self) -> $ float_type { unsafe { core :: mem :: transmute (self) } } } # [doc = " Loads a value from the atomic float."] # [doc = ""] # [doc = " `load` takes an [`Ordering`] argument which describes the memory ordering of this operation."] # [doc = " Possible values are [`SeqCst`], [`Acquire`] and [`Relaxed`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `order` is [`Release`] or [`AcqRel`]."] # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn load (& self , order : Ordering) -> $ float_type { self . inner . load (order) } # [doc = " Stores a value into the atomic float."] # [doc = ""] # [doc = " `store` takes an [`Ordering`] argument which describes the memory ordering of this operation."] # [doc = "  Possible values are [`SeqCst`], [`Release`] and [`Relaxed`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `order` is [`Acquire`] or [`AcqRel`]."] # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn store (& self , val : $ float_type , order : Ordering) { self . inner . store (val , order) } cfg_has_atomic_cas_or_amo32 ! { # [doc = " Stores a value into the atomic float, returning the previous value."] # [doc = ""] # [doc = " `swap` takes an [`Ordering`] argument which describes the memory ordering"] # [doc = " of this operation. All ordering modes are possible. Note that using"] # [doc = " [`Acquire`] makes the store part of this operation [`Relaxed`], and"] # [doc = " using [`Release`] makes the load part [`Relaxed`]."] # [inline] # [cfg_attr (miri , track_caller)] pub fn swap (& self , val : $ float_type , order : Ordering) -> $ float_type { self . inner . swap (val , order) } cfg_has_atomic_cas ! { # [doc = " Stores a value into the atomic float if the current value is the same as"] # [doc = " the `current` value."] # [doc = ""] # [doc = " The return value is a result indicating whether the new value was written and"] # [doc = " containing the previous value. On success this value is guaranteed to be equal to"] # [doc = " `current`."] # [doc = ""] # [doc = " `compare_exchange` takes two [`Ordering`] arguments to describe the memory"] # [doc = " ordering of this operation. `success` describes the required ordering for the"] # [doc = " read-modify-write operation that takes place if the comparison with `current` succeeds."] # [doc = " `failure` describes the required ordering for the load operation that takes place when"] # [doc = " the comparison fails. Using [`Acquire`] as success ordering makes the store part"] # [doc = " of this operation [`Relaxed`], and using [`Release`] makes the successful load"] # [doc = " [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `failure` is [`Release`], [`AcqRel`]."] # [cfg_attr (docsrs , doc (alias = "compare_and_swap"))] # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn compare_exchange (& self , current : $ float_type , new : $ float_type , success : Ordering , failure : Ordering ,) -> Result <$ float_type , $ float_type > { self . inner . compare_exchange (current , new , success , failure) } # [doc = " Stores a value into the atomic float if the current value is the same as"] # [doc = " the `current` value."] # [doc = " Unlike [`compare_exchange`](Self::compare_exchange)"] # [doc = " this function is allowed to spuriously fail even"] # [doc = " when the comparison succeeds, which can result in more efficient code on some"] # [doc = " platforms. The return value is a result indicating whether the new value was"] # [doc = " written and containing the previous value."] # [doc = ""] # [doc = " `compare_exchange_weak` takes two [`Ordering`] arguments to describe the memory"] # [doc = " ordering of this operation. `success` describes the required ordering for the"] # [doc = " read-modify-write operation that takes place if the comparison with `current` succeeds."] # [doc = " `failure` describes the required ordering for the load operation that takes place when"] # [doc = " the comparison fails. Using [`Acquire`] as success ordering makes the store part"] # [doc = " of this operation [`Relaxed`], and using [`Release`] makes the successful load"] # [doc = " [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `failure` is [`Release`], [`AcqRel`]."] # [cfg_attr (docsrs , doc (alias = "compare_and_swap"))] # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn compare_exchange_weak (& self , current : $ float_type , new : $ float_type , success : Ordering , failure : Ordering ,) -> Result <$ float_type , $ float_type > { self . inner . compare_exchange_weak (current , new , success , failure) } # [doc = " Adds to the current value, returning the previous value."] # [doc = ""] # [doc = " This operation wraps around on overflow."] # [doc = ""] # [doc = " `fetch_add` takes an [`Ordering`] argument which describes the memory ordering"] # [doc = " of this operation. All ordering modes are possible. Note that using"] # [doc = " [`Acquire`] makes the store part of this operation [`Relaxed`], and"] # [doc = " using [`Release`] makes the load part [`Relaxed`]."] # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_add (& self , val : $ float_type , order : Ordering) -> $ float_type { self . inner . fetch_add (val , order) } # [doc = " Subtracts from the current value, returning the previous value."] # [doc = ""] # [doc = " This operation wraps around on overflow."] # [doc = ""] # [doc = " `fetch_sub` takes an [`Ordering`] argument which describes the memory ordering"] # [doc = " of this operation. All ordering modes are possible. Note that using"] # [doc = " [`Acquire`] makes the store part of this operation [`Relaxed`], and"] # [doc = " using [`Release`] makes the load part [`Relaxed`]."] # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_sub (& self , val : $ float_type , order : Ordering) -> $ float_type { self . inner . fetch_sub (val , order) } # [doc = " Fetches the value, and applies a function to it that returns an optional"] # [doc = " new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else"] # [doc = " `Err(previous_value)`."] # [doc = ""] # [doc = " Note: This may call the function multiple times if the value has been changed from other threads in"] # [doc = " the meantime, as long as the function returns `Some(_)`, but the function will have been applied"] # [doc = " only once to the stored value."] # [doc = ""] # [doc = " `fetch_update` takes two [`Ordering`] arguments to describe the memory ordering of this operation."] # [doc = " The first describes the required ordering for when the operation finally succeeds while the second"] # [doc = " describes the required ordering for loads. These correspond to the success and failure orderings of"] # [doc = " [`compare_exchange`](Self::compare_exchange) respectively."] # [doc = ""] # [doc = " Using [`Acquire`] as success ordering makes the store part"] # [doc = " of this operation [`Relaxed`], and using [`Release`] makes the final successful load"] # [doc = " [`Relaxed`]. The (failed) load ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `fetch_order` is [`Release`], [`AcqRel`]."] # [doc = ""] # [doc = " # Considerations"] # [doc = ""] # [doc = " This method is not magic; it is not provided by the hardware."] # [doc = " It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),"] # [doc = " and suffers from the same drawbacks."] # [doc = " In particular, this method will not circumvent the [ABA Problem]."] # [doc = ""] # [doc = " [ABA Problem]: https://en.wikipedia.org/wiki/ABA_problem"] # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub fn fetch_update < F > (& self , set_order : Ordering , fetch_order : Ordering , mut f : F ,) -> Result <$ float_type , $ float_type > where F : FnMut ($ float_type) -> Option <$ float_type >, { let mut prev = self . load (fetch_order) ; while let Some (next) = f (prev) { match self . compare_exchange_weak (prev , next , set_order , fetch_order) { x @ Ok (_) => return x , Err (next_prev) => prev = next_prev , } } Err (prev) } # [doc = " Maximum with the current value."] # [doc = ""] # [doc = " Finds the maximum of the current value and the argument `val`, and"] # [doc = " sets the new value to the result."] # [doc = ""] # [doc = " Returns the previous value."] # [doc = ""] # [doc = " `fetch_max` takes an [`Ordering`] argument which describes the memory ordering"] # [doc = " of this operation. All ordering modes are possible. Note that using"] # [doc = " [`Acquire`] makes the store part of this operation [`Relaxed`], and"] # [doc = " using [`Release`] makes the load part [`Relaxed`]."] # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_max (& self , val : $ float_type , order : Ordering) -> $ float_type { self . inner . fetch_max (val , order) } # [doc = " Minimum with the current value."] # [doc = ""] # [doc = " Finds the minimum of the current value and the argument `val`, and"] # [doc = " sets the new value to the result."] # [doc = ""] # [doc = " Returns the previous value."] # [doc = ""] # [doc = " `fetch_min` takes an [`Ordering`] argument which describes the memory ordering"] # [doc = " of this operation. All ordering modes are possible. Note that using"] # [doc = " [`Acquire`] makes the store part of this operation [`Relaxed`], and"] # [doc = " using [`Release`] makes the load part [`Relaxed`]."] # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_min (& self , val : $ float_type , order : Ordering) -> $ float_type { self . inner . fetch_min (val , order) } } # [doc = " Negates the current value, and sets the new value to the result."] # [doc = ""] # [doc = " Returns the previous value."] # [doc = ""] # [doc = " `fetch_neg` takes an [`Ordering`] argument which describes the memory ordering"] # [doc = " of this operation. All ordering modes are possible. Note that using"] # [doc = " [`Acquire`] makes the store part of this operation [`Relaxed`], and"] # [doc = " using [`Release`] makes the load part [`Relaxed`]."] # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_neg (& self , order : Ordering) -> $ float_type { self . inner . fetch_neg (order) } # [doc = " Computes the absolute value of the current value, and sets the"] # [doc = " new value to the result."] # [doc = ""] # [doc = " Returns the previous value."] # [doc = ""] # [doc = " `fetch_abs` takes an [`Ordering`] argument which describes the memory ordering"] # [doc = " of this operation. All ordering modes are possible. Note that using"] # [doc = " [`Acquire`] makes the store part of this operation [`Relaxed`], and"] # [doc = " using [`Release`] makes the load part [`Relaxed`]."] # [inline] # [cfg_attr (miri , track_caller)] pub fn fetch_abs (& self , order : Ordering) -> $ float_type { self . inner . fetch_abs (order) } } # [cfg (not (portable_atomic_no_const_raw_ptr_deref))] doc_comment ! { concat ! ("Raw transmutation to `&" , stringify ! ($ atomic_int_type) , "`.

See [`" , stringify ! ($ float_type) , "::from_bits`] for some discussion of the
portability of this operation (there are almost no issues).

This is `const fn` on Rust 1.58+.") , # [inline] pub const fn as_bits (& self) -> &$ atomic_int_type { self . inner . as_bits () } } # [cfg (portable_atomic_no_const_raw_ptr_deref)] doc_comment ! { concat ! ("Raw transmutation to `&" , stringify ! ($ atomic_int_type) , "`.

See [`" , stringify ! ($ float_type) , "::from_bits`] for some discussion of the
portability of this operation (there are almost no issues).

This is `const fn` on Rust 1.58+.") , # [inline] pub fn as_bits (& self) -> &$ atomic_int_type { self . inner . as_bits () } } const_fn ! { const_if : # [cfg (not (portable_atomic_no_const_raw_ptr_deref))] ; # [doc = " Returns a mutable pointer to the underlying float."] # [doc = ""] # [doc = " Returning an `*mut` pointer from a shared reference to this atomic is"] # [doc = " safe because the atomic types work with interior mutability. Any use of"] # [doc = " the returned raw pointer requires an `unsafe` block and has to uphold"] # [doc = " the safety requirements. If there is concurrent access, note the following"] # [doc = " additional safety requirements:"] # [doc = ""] # [doc = " - If this atomic type is [lock-free](Self::is_lock_free), any concurrent"] # [doc = "   operations on it must be atomic."] # [doc = " - Otherwise, any concurrent operations on it must be compatible with"] # [doc = "   operations performed by this atomic type."] # [doc = ""] # [doc = " This is `const fn` on Rust 1.58+."] # [inline] pub const fn as_ptr (& self) -> * mut $ float_type { self . inner . as_ptr () } } } # [cfg (not (feature = "require-cas"))] cfg_no_atomic_cas ! { # [doc (hidden)] # [allow (unused_variables , clippy :: unused_self , clippy :: extra_unused_lifetimes)] impl <'a > $ atomic_type { cfg_no_atomic_cas_or_amo32 ! { # [inline] pub fn swap (& self , val : $ float_type , order : Ordering) -> $ float_type where &'a Self : HasSwap , { unimplemented ! () } } # [inline] pub fn compare_exchange (& self , current : $ float_type , new : $ float_type , success : Ordering , failure : Ordering ,) -> Result <$ float_type , $ float_type > where &'a Self : HasCompareExchange , { unimplemented ! () } # [inline] pub fn compare_exchange_weak (& self , current : $ float_type , new : $ float_type , success : Ordering , failure : Ordering ,) -> Result <$ float_type , $ float_type > where &'a Self : HasCompareExchangeWeak , { unimplemented ! () } # [inline] pub fn fetch_add (& self , val : $ float_type , order : Ordering) -> $ float_type where &'a Self : HasFetchAdd , { unimplemented ! () } # [inline] pub fn fetch_sub (& self , val : $ float_type , order : Ordering) -> $ float_type where &'a Self : HasFetchSub , { unimplemented ! () } # [inline] pub fn fetch_update < F > (& self , set_order : Ordering , fetch_order : Ordering , f : F ,) -> Result <$ float_type , $ float_type > where F : FnMut ($ float_type) -> Option <$ float_type >, &'a Self : HasFetchUpdate , { unimplemented ! () } # [inline] pub fn fetch_max (& self , val : $ float_type , order : Ordering) -> $ float_type where &'a Self : HasFetchMax , { unimplemented ! () } # [inline] pub fn fetch_min (& self , val : $ float_type , order : Ordering) -> $ float_type where &'a Self : HasFetchMin , { unimplemented ! () } cfg_no_atomic_cas_or_amo32 ! { # [inline] pub fn fetch_neg (& self , order : Ordering) -> $ float_type where &'a Self : HasFetchNeg , { unimplemented ! () } # [inline] pub fn fetch_abs (& self , order : Ordering) -> $ float_type where &'a Self : HasFetchAbs , { unimplemented ! () } } } } } ; }
};
}
