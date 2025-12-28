macro_rules! deps {
    () => {
        UnsafeCell!();
        Reading!();
    };
}

macro_rules! ConstPtr {
    () => {
        deps!();
        # [doc = " A checked immutable raw pointer to an [`UnsafeCell`]."] # [doc = ""] # [doc = " This type is essentially a [`*const T`], but with the added ability to"] # [doc = " participate in Loom's [`UnsafeCell`] access tracking. While a `ConstPtr` to a"] # [doc = " given [`UnsafeCell`] exists, Loom will track that the [`UnsafeCell`] is"] # [doc = " being accessed immutably."] # [doc = ""] # [doc = " [`ConstPtr`]s are produced by the [`UnsafeCell::get`] method. The pointed"] # [doc = " value can be accessed using [`ConstPtr::deref`]."] # [doc = ""] # [doc = " Any number of [`ConstPtr`]s may concurrently access a given [`UnsafeCell`]."] # [doc = " However, if the [`UnsafeCell`] is accessed mutably (by"] # [doc = " [`UnsafeCell::with_mut`] or [`UnsafeCell::get_mut`]) while a [`ConstPtr`]"] # [doc = " exists, Loom will detect the concurrent mutable and immutable accesses and"] # [doc = " panic."] # [doc = ""] # [doc = " Note that the cell is considered to be immutably accessed for *the entire"] # [doc = " lifespan of the `ConstPtr`*, not just when the `ConstPtr` is actively"] # [doc = " dereferenced."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Although the `ConstPtr` type is checked for concurrent access violations, it"] # [doc = " is **still a raw pointer**. A `ConstPtr` is not bound to the lifetime of the"] # [doc = " [`UnsafeCell`] from which it was produced, and may outlive the cell. Loom"] # [doc = " does *not* currently check for dangling pointers. Therefore, the user is"] # [doc = " responsible for ensuring that a `ConstPtr` does not dangle. However, unlike"] # [doc = " a normal `*const T`, `ConstPtr`s may only be produced from a valid"] # [doc = " [`UnsafeCell`], and therefore can be assumed to never be null."] # [doc = ""] # [doc = " Additionally, it is possible to write code in which raw pointers to an"] # [doc = " [`UnsafeCell`] are constructed that are *not* checked by Loom. If a raw"] # [doc = " pointer \"escapes\" Loom's tracking, invalid accesses may not be detected,"] # [doc = " resulting in tests passing when they should have failed. See [here] for"] # [doc = " details on how to avoid accidentally escaping the model."] # [doc = ""] # [doc = " [`*const T`]: https://doc.rust-lang.org/stable/std/primitive.pointer.html"] # [doc = " [here]: #correct-usage"] # [derive (Debug)] pub struct ConstPtr < T : ? Sized > { # [doc = " Drop guard representing the lifetime of the `ConstPtr`'s access."] _guard : rt :: cell :: Reading , ptr : * const T , }
    };
}

ConstPtr!();