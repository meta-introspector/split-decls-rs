macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! Symbol {
    () => {
        deps!();
        # [doc = " Symbol from a library."] # [doc = ""] # [doc = " This type is a safeguard against using dynamically loaded symbols after a `Library` is"] # [doc = " unloaded. The primary method to create an instance of a `Symbol` is via [`Library::get`]."] # [doc = ""] # [doc = " The `Deref` trait implementation allows the use of `Symbol` as if it was a function or variable"] # [doc = " itself, without taking care to “extract” the function or variable manually most of the time."] # [doc = ""] # [doc = " [`Library::get`]: Library::get"] # [cfg_attr (libloading_docs , doc (cfg (any (unix , windows))))] pub struct Symbol < 'lib , T : 'lib > { inner : imp :: Symbol < T > , pd : marker :: PhantomData < & 'lib T > , }
    };
}

Symbol!()