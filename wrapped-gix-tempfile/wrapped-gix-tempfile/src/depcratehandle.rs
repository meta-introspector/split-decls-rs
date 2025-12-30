// Generated macro for Handle (struct)
macro_rules! DepcrateHandle {
() => {
// Module: crate
// Provides: {"Handle"}
// Dependencies: {}
# [doc = " A registered temporary file which will delete itself on drop or if the program is receiving signals that"] # [doc = " should cause it to terminate."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " Signals interrupting the calling thread right after taking ownership of the registered tempfile"] # [doc = " will cause all but this tempfile to be removed automatically. In the common case it will persist on disk as destructors"] # [doc = " were not called or didn't get to remove the file."] # [doc = ""] # [doc = " In the best case the file is a true temporary with a non-clashing name that 'only' fills up the disk,"] # [doc = " in the worst case the temporary file is used as a lock file which may leave the repository in a locked"] # [doc = " state forever."] # [doc = ""] # [doc = " This kind of raciness exists whenever [`take()`][Handle::take()] is used and can't be circumvented."] # [derive (Debug)] # [must_use = "A handle that is immediately dropped doesn't lock a resource meaningfully"] pub struct Handle < Marker : std :: fmt :: Debug > { id : usize , _marker : PhantomData < Marker > , }
};
}
