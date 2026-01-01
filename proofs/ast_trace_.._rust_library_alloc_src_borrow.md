# AST Trace: ../rust/library/alloc/src/borrow.rs

Generated 32 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=6

```rust
//! A module for working with borrowed data.

#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
pub use core::borrow::{Borrow, BorrowMut};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
#[cfg(not(no_global_oom_handling))]
use core::ops::{Add, AddAssign};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use core::ops::{Deref, DerefPure};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=borrow | COMPLEXITY=6 | LINES=19

```rust
use Cow::*;

use crate::fmt;
#[cfg(not(no_global_oom_handling))]
use crate::string::String;

// FIXME(inference): const bounds removed due to inference regressions found by crater;
//   see https://github.com/rust-lang/rust/issues/147964
// #[rustc_const_unstable(feature = "const_convert", issue = "143773")]
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, B: ?Sized + ToOwned> Borrow<B> for Cow<'a, B>
// where
//     B::Owned: [const] Borrow<B>,
{
    fn borrow(&self) -> &B {
        &**self
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=to_owned | COMPLEXITY=8 | LINES=52

```rust
/// A generalization of `Clone` to borrowed data.
///
/// Some types make it possible to go from borrowed to owned, usually by
/// implementing the `Clone` trait. But `Clone` works only for going from `&T`
/// to `T`. The `ToOwned` trait generalizes `Clone` to construct owned data
/// from any borrow of a given type.
#[rustc_diagnostic_item = "ToOwned"]
#[stable(feature = "rust1", since = "1.0.0")]
pub trait ToOwned {
    /// The resulting type after obtaining ownership.
    #[stable(feature = "rust1", since = "1.0.0")]
    type Owned: Borrow<Self>;

    /// Creates owned data from borrowed data, usually by cloning.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s: &str = "a";
    /// let ss: String = s.to_owned();
    ///
    /// let v: &[i32] = &[1, 2];
    /// let vv: Vec<i32> = v.to_owned();
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use = "cloning is often expensive and is not expected to have side effects"]
    #[rustc_diagnostic_item = "to_owned_method"]
    fn to_owned(&self) -> Self::Owned;

    /// Uses borrowed data to replace owned data, usually by cloning.
    ///
    /// This is borrow-generalized version of [`Clone::clone_from`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s: String = String::new();
    /// "hello".clone_into(&mut s);
    ///
    /// let mut v: Vec<i32> = Vec::new();
    /// [1, 2][..].clone_into(&mut v);
    /// ```
    #[stable(feature = "toowned_clone_into", since = "1.63.0")]
    fn clone_into(&self, target: &mut Self::Owned) {
        *target = self.to_owned();
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=to_owned | COMPLEXITY=6 | LINES=15

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ToOwned for T
where
    T: Clone,
{
    type Owned = T;
    fn to_owned(&self) -> T {
        self.clone()
    }

    fn clone_into(&self, target: &mut T) {
        target.clone_from(self);
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=32

```rust
/// A clone-on-write smart pointer.
///
/// The type `Cow` is a smart pointer providing clone-on-write functionality: it
/// can enclose and provide immutable access to borrowed data, and clone the
/// data lazily when mutation or ownership is required. The type is designed to
/// work with general borrowed data via the `Borrow` trait.
///
/// `Cow` implements `Deref`, which means that you can call
/// non-mutating methods directly on the data it encloses. If mutation
/// is desired, `to_mut` will obtain a mutable reference to an owned
/// value, cloning if necessary.
///
/// If you need reference-counting pointers, note that
/// [`Rc::make_mut`][crate::rc::Rc::make_mut] and
/// [`Arc::make_mut`][crate::sync::Arc::make_mut] can provide clone-on-write
/// functionality as well.
///
/// # Examples
///
/// ```
/// use std::borrow::Cow;
///
/// fn abs_all(input: &mut Cow<'_, [i32]>) {
///     for i in 0..input.len() {
///         let v = input[i];
///         if v < 0 {
///             // Clones into a vector if not already owned.
///             input.to_mut()[i] = -v;
///         }
///     }
/// }
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=24

```rust
///
/// // No clone occurs because `input` doesn't need to be mutated.
/// let slice = [0, 1, 2];
/// let mut input = Cow::from(&slice[..]);
/// abs_all(&mut input);
///
/// // Clone occurs because `input` needs to be mutated.
/// let slice = [-1, 0, 1];
/// let mut input = Cow::from(&slice[..]);
/// abs_all(&mut input);
///
/// // No clone occurs because `input` is already owned.
/// let mut input = Cow::from(vec![-1, 0, 1]);
/// abs_all(&mut input);
/// ```
///
/// Another example showing how to keep `Cow` in a struct:
///
/// ```
/// use std::borrow::Cow;
///
/// struct Items<'a, X> where [X]: ToOwned<Owned = Vec<X>> {
///     values: Cow<'a, [X]>,
/// }
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=6

```rust
///
/// impl<'a, X: Clone + 'a> Items<'a, X> where [X]: ToOwned<Owned = Vec<X>> {
///     fn new(v: Cow<'a, [X]>) -> Self {
///         Items { values: v }
///     }
/// }
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=7 | LINES=8

```rust
///
/// // Creates a container from borrowed values of a slice
/// let readonly = [1, 2];
/// let borrowed = Items::new((&readonly[..]).into());
/// match borrowed {
///     Items { values: Cow::Borrowed(b) } => println!("borrowed {b:?}"),
///     _ => panic!("expect borrowed value"),
/// }
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=7 | LINES=11

```rust
///
/// let mut clone_on_write = borrowed;
/// // Mutates the data from slice into owned vec and pushes a new value on top
/// clone_on_write.values.to_mut().push(3);
/// println!("clone_on_write = {:?}", clone_on_write.values);
///
/// // The data was mutated. Let's check it out.
/// match clone_on_write {
///     Items { values: Cow::Owned(_) } => println!("clone_on_write contains owned data"),
///     _ => panic!("expect owned data"),
/// }
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=15

```rust
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_diagnostic_item = "Cow"]
pub enum Cow<'a, B: ?Sized + 'a>
where
    B: ToOwned,
{
    /// Borrowed data.
    #[stable(feature = "rust1", since = "1.0.0")]
    Borrowed(#[stable(feature = "rust1", since = "1.0.0")] &'a B),

    /// Owned data.
    #[stable(feature = "rust1", since = "1.0.0")]
    Owned(#[stable(feature = "rust1", since = "1.0.0")] <B as ToOwned>::Owned),
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=16 | LINES=20

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<B: ?Sized + ToOwned> Clone for Cow<'_, B> {
    fn clone(&self) -> Self {
        match *self {
            Borrowed(b) => Borrowed(b),
            Owned(ref o) => {
                let b: &B = o.borrow();
                Owned(b.to_owned())
            }
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (&mut Owned(ref mut dest), &Owned(ref o)) => o.borrow().clone_into(dest),
            (t, s) => *t = s.clone(),
        }
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=to_mut | COMPLEXITY=41 | LINES=116

```rust
impl<B: ?Sized + ToOwned> Cow<'_, B> {
    /// Returns true if the data is borrowed, i.e. if `to_mut` would require additional work.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(cow_is_borrowed)]
    /// use std::borrow::Cow;
    ///
    /// let cow = Cow::Borrowed("moo");
    /// assert!(cow.is_borrowed());
    ///
    /// let bull: Cow<'_, str> = Cow::Owned("...moo?".to_string());
    /// assert!(!bull.is_borrowed());
    /// ```
    #[unstable(feature = "cow_is_borrowed", issue = "65143")]
    pub const fn is_borrowed(&self) -> bool {
        match *self {
            Borrowed(_) => true,
            Owned(_) => false,
        }
    }

    /// Returns true if the data is owned, i.e. if `to_mut` would be a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(cow_is_borrowed)]
    /// use std::borrow::Cow;
    ///
    /// let cow: Cow<'_, str> = Cow::Owned("moo".to_string());
    /// assert!(cow.is_owned());
    ///
    /// let bull = Cow::Borrowed("...moo?");
    /// assert!(!bull.is_owned());
    /// ```
    #[unstable(feature = "cow_is_borrowed", issue = "65143")]
    pub const fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    /// Acquires a mutable reference to the owned form of the data.
    ///
    /// Clones the data if it is not already owned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    ///
    /// let mut cow = Cow::Borrowed("foo");
    /// cow.to_mut().make_ascii_uppercase();
    ///
    /// assert_eq!(
    ///   cow,
    ///   Cow::Owned(String::from("FOO")) as Cow<'_, str>
    /// );
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
        match *self {
            Borrowed(borrowed) => {
                *self = Owned(borrowed.to_owned());
                match *self {
                    Borrowed(..) => unreachable!(),
                    Owned(ref mut owned) => owned,
                }
            }
            Owned(ref mut owned) => owned,
        }
    }

    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already owned.
    ///
    /// # Examples
    ///
    /// Calling `into_owned` on a `Cow::Borrowed` returns a clone of the borrowed data:
    ///
    /// ```
    /// use std::borrow::Cow;
    ///
    /// let s = "Hello world!";
    /// let cow = Cow::Borrowed(s);
    ///
    /// assert_eq!(
    ///   cow.into_owned(),
    ///   String::from(s)
    /// );
    /// ```
    ///
    /// Calling `into_owned` on a `Cow::Owned` returns the owned data. The data is moved out of the
    /// `Cow` without being cloned.
    ///
    /// ```
    /// use std::borrow::Cow;
    ///
    /// let s = "Hello world!";
    /// let cow: Cow<'_, str> = Cow::Owned(String::from(s));
    ///
    /// assert_eq!(
    ///   cow.into_owned(),
    ///   String::from(s)
    /// );
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn into_owned(self) -> <B as ToOwned>::Owned {
        match self {
            Borrowed(borrowed) => borrowed.to_owned(),
            Owned(owned) => owned,
        }
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=deref | COMPLEXITY=10 | LINES=19

```rust
// FIXME(inference): const bounds removed due to inference regressions found by crater;
//   see https://github.com/rust-lang/rust/issues/147964
// #[rustc_const_unstable(feature = "const_convert", issue = "143773")]
#[stable(feature = "rust1", since = "1.0.0")]
impl<B: ?Sized + ToOwned> Deref for Cow<'_, B>
where
    B::Owned: Borrow<B>,
//     B::Owned: [const] Borrow<B>,
{
    type Target = B;

    fn deref(&self) -> &B {
        match *self {
            Borrowed(borrowed) => borrowed,
            Owned(ref owned) => owned.borrow(),
        }
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=12 | LINES=7

```rust
// `Cow<'_, T>` can only implement `DerefPure` if `<T::Owned as Borrow<T>>` (and `BorrowMut<T>`) is trusted.
// For now, we restrict `DerefPure for Cow<T>` to `T: Sized` (`T as Borrow<T>` is trusted),
// `str` (`String as Borrow<str>` is trusted) and `[T]` (`Vec<T> as Borrow<[T]>` is trusted).
// In the future, a `BorrowPure<T>` trait analogous to `DerefPure` might generalize this.
#[unstable(feature = "deref_pure_trait", issue = "87121")]
unsafe impl<T: Clone> DerefPure for Cow<'_, T> {}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[cfg(not(no_global_oom_handling))]
#[unstable(feature = "deref_pure_trait", issue = "87121")]
unsafe impl DerefPure for Cow<'_, str> {}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[cfg(not(no_global_oom_handling))]
#[unstable(feature = "deref_pure_trait", issue = "87121")]
unsafe impl<T: Clone> DerefPure for Cow<'_, [T]> {}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<B: ?Sized> Eq for Cow<'_, B> where B: Eq + ToOwned {}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=cmp | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<B: ?Sized> Ord for Cow<'_, B>
where
    B: Ord + ToOwned,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=5 | LINES=12

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, 'b, B: ?Sized, C: ?Sized> PartialEq<Cow<'b, C>> for Cow<'a, B>
where
    B: PartialEq<C> + ToOwned,
    C: ToOwned,
{
    #[inline]
    fn eq(&self, other: &Cow<'b, C>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=partial_cmp | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, B: ?Sized> PartialOrd for Cow<'a, B>
where
    B: PartialOrd + ToOwned,
{
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, B>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=9 | LINES=13

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<B: ?Sized> fmt::Debug for Cow<'_, B>
where
    B: fmt::Debug + ToOwned<Owned: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Borrowed(ref b) => fmt::Debug::fmt(b, f),
            Owned(ref o) => fmt::Debug::fmt(o, f),
        }
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=9 | LINES=13

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<B: ?Sized> fmt::Display for Cow<'_, B>
where
    B: fmt::Display + ToOwned<Owned: fmt::Display>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Borrowed(ref b) => fmt::Display::fmt(b, f),
            Owned(ref o) => fmt::Display::fmt(o, f),
        }
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=default | COMPLEXITY=7 | LINES=11

```rust
#[stable(feature = "default", since = "1.11.0")]
impl<B: ?Sized> Default for Cow<'_, B>
where
    B: ToOwned<Owned: Default>,
{
    /// Creates an owned Cow<'a, B> with the default value for the contained owned value.
    fn default() -> Self {
        Owned(<B as ToOwned>::Owned::default())
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=hash | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<B: ?Sized> Hash for Cow<'_, B>
where
    B: Hash + ToOwned,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=as_ref | COMPLEXITY=5 | LINES=13

```rust
// FIXME(inference): const bounds removed due to inference regressions found by crater;
//   see https://github.com/rust-lang/rust/issues/147964
// #[rustc_const_unstable(feature = "const_convert", issue = "143773")]
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized + ToOwned> AsRef<T> for Cow<'_, T>
// where
//     T::Owned: [const] Borrow<T>,
{
    fn as_ref(&self) -> &T {
        self
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=add | COMPLEXITY=5 | LINES=12

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "cow_add", since = "1.14.0")]
impl<'a> Add<&'a str> for Cow<'a, str> {
    type Output = Cow<'a, str>;

    #[inline]
    fn add(mut self, rhs: &'a str) -> Self::Output {
        self += rhs;
        self
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=add | COMPLEXITY=5 | LINES=12

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "cow_add", since = "1.14.0")]
impl<'a> Add<Cow<'a, str>> for Cow<'a, str> {
    type Output = Cow<'a, str>;

    #[inline]
    fn add(mut self, rhs: Cow<'a, str>) -> Self::Output {
        self += rhs;
        self
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=add_assign | COMPLEXITY=15 | LINES=17

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "cow_add", since = "1.14.0")]
impl<'a> AddAssign<&'a str> for Cow<'a, str> {
    fn add_assign(&mut self, rhs: &'a str) {
        if self.is_empty() {
            *self = Cow::Borrowed(rhs)
        } else if !rhs.is_empty() {
            if let Cow::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = Cow::Owned(s);
            }
            self.to_mut().push_str(rhs);
        }
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=add_assign | COMPLEXITY=15 | LINES=17

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "cow_add", since = "1.14.0")]
impl<'a> AddAssign<Cow<'a, str>> for Cow<'a, str> {
    fn add_assign(&mut self, rhs: Cow<'a, str>) {
        if self.is_empty() {
            *self = rhs
        } else if !rhs.is_empty() {
            if let Cow::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = Cow::Owned(s);
            }
            self.to_mut().push_str(&rhs);
        }
    }
}
```

---
*Generated by AST tracing system*
