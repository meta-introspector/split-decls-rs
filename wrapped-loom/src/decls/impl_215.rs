macro_rules! deps {
    () => {
        UnsafeCell!();
        ConstPtr!();
        MutPtr!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < T : ? Sized > UnsafeCell < T > { # [doc = " Get an immutable pointer to the wrapped value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if the access is not valid under the Rust memory"] # [doc = " model."] # [track_caller] pub fn with < F , R > (& self , f : F) -> R where F : FnOnce (* const T) -> R , { let _reading = self . state . start_read (location ! ()) ; f (self . data . get () as * const T) } # [doc = " Get a mutable pointer to the wrapped value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if the access is not valid under the Rust memory"] # [doc = " model."] # [track_caller] pub fn with_mut < F , R > (& self , f : F) -> R where F : FnOnce (* mut T) -> R , { let _writing = self . state . start_write (location ! ()) ; f (self . data . get ()) } # [doc = " Get an immutable pointer to the wrapped value."] # [doc = ""] # [doc = " This function returns a [`ConstPtr`] guard, which is analogous to a"] # [doc = " `*const T`, but tracked by Loom. As long as the returned `ConstPtr`"] # [doc = " exists, Loom will consider the cell to be accessed immutably."] # [doc = ""] # [doc = " This means that any mutable accesses (e.g. calls to [`with_mut`] or"] # [doc = " [`get_mut`]) while the returned guard is live will result in a panic."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if the access is not valid under the Rust memory"] # [doc = " model."] # [doc = ""] # [doc = " [`with_mut`]: UnsafeCell::with_mut"] # [doc = " [`get_mut`]: UnsafeCell::get_mut"] # [track_caller] pub fn get (& self) -> ConstPtr < T > { ConstPtr { _guard : self . state . start_read (location ! ()) , ptr : self . data . get () , } } # [doc = " Get a mutable pointer to the wrapped value."] # [doc = ""] # [doc = " This function returns a [`MutPtr`] guard, which is analogous to a"] # [doc = " `*mut T`, but tracked by Loom. As long as the returned `MutPtr`"] # [doc = " exists, Loom will consider the cell to be accessed mutably."] # [doc = ""] # [doc = " This means that any concurrent mutable or immutable accesses (e.g. calls"] # [doc = " to [`with`], [`with_mut`], [`get`], or [`get_mut`]) while the returned"] # [doc = " guard is live will result in a panic."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if the access is not valid under the Rust memory"] # [doc = " model."] # [doc = ""] # [doc = " [`with`]: UnsafeCell::with"] # [doc = " [`with_mut`]: UnsafeCell::with_mut"] # [doc = " [`get`]: UnsafeCell::get"] # [doc = " [`get_mut`]: UnsafeCell::get_mut"] # [track_caller] pub fn get_mut (& self) -> MutPtr < T > { MutPtr { _guard : self . state . start_write (location ! ()) , ptr : self . data . get () , } } }
    };
}

impl_215!()