mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool , AtomicUsize , Ordering } ;}
mkuse!{use crate :: { ops , process } ;}
mkitem!{mkstruct!{# [doc = " Reference counter internals."] struct Counter < C > { # [doc = " The number of senders associated with the channel."] senders : Atomic < usize > , # [doc = " The number of receivers associated with the channel."] receivers : Atomic < usize > , # [doc = " Set to `true` if the last sender or the last receiver reference deallocates the channel."] destroy : Atomic < bool > , # [doc = " The internal channel."] chan : C , }}}

macro_rules! new_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new in module {}", module_path!());
    };
}

mkfn!{
    new_introspect!();
    # [doc = " Wraps a channel into the reference counter."] pub (crate) fn new < C > (chan : C) -> (Sender < C > , Receiver < C >) { let counter = Box :: into_raw (Box :: new (Counter { senders : AtomicUsize :: new (1) , receivers : AtomicUsize :: new (1) , destroy : AtomicBool :: new (false) , chan , })) ; let s = Sender { counter } ; let r = Receiver { counter } ; (s , r) }
}
mkitem!{mkstruct!{# [doc = " The sending side."] pub (crate) struct Sender < C > { counter : * mut Counter < C > , }}}
mkitem!{mkimpl!{impl < C > Sender < C > { # [doc = " Returns the internal `Counter`."] fn counter (& self) -> & Counter < C > { unsafe { & * self . counter } } # [doc = " Acquires another sender reference."] pub (crate) fn acquire (& self) -> Sender < C > { let count = self . counter () . senders . fetch_add (1 , Ordering :: Relaxed) ; if count > isize :: MAX as usize { process :: abort () ; } Sender { counter : self . counter } } # [doc = " Releases the sender reference."] # [doc = ""] # [doc = " Function `disconnect` will be called if this is the last sender reference."] pub (crate) unsafe fn release < F : FnOnce (& C) -> bool > (& self , disconnect : F) { if self . counter () . senders . fetch_sub (1 , Ordering :: AcqRel) == 1 { disconnect (& self . counter () . chan) ; if self . counter () . destroy . swap (true , Ordering :: AcqRel) { drop (unsafe { Box :: from_raw (self . counter) }) ; } } } }}}
mkitem!{mkimpl!{impl < C > ops :: Deref for Sender < C > { type Target = C ; fn deref (& self) -> & C { & self . counter () . chan } }}}
mkitem!{mkimpl!{impl < C > PartialEq for Sender < C > { fn eq (& self , other : & Sender < C >) -> bool { self . counter == other . counter } }}}
mkitem!{mkstruct!{# [doc = " The receiving side."] pub (crate) struct Receiver < C > { counter : * mut Counter < C > , }}}
mkitem!{mkimpl!{impl < C > Receiver < C > { # [doc = " Returns the internal `Counter`."] fn counter (& self) -> & Counter < C > { unsafe { & * self . counter } } # [doc = " Acquires another receiver reference."] pub (crate) fn acquire (& self) -> Receiver < C > { let count = self . counter () . receivers . fetch_add (1 , Ordering :: Relaxed) ; if count > isize :: MAX as usize { process :: abort () ; } Receiver { counter : self . counter } } # [doc = " Releases the receiver reference."] # [doc = ""] # [doc = " Function `disconnect` will be called if this is the last receiver reference."] pub (crate) unsafe fn release < F : FnOnce (& C) -> bool > (& self , disconnect : F) { if self . counter () . receivers . fetch_sub (1 , Ordering :: AcqRel) == 1 { disconnect (& self . counter () . chan) ; if self . counter () . destroy . swap (true , Ordering :: AcqRel) { drop (unsafe { Box :: from_raw (self . counter) }) ; } } } }}}
mkitem!{mkimpl!{impl < C > ops :: Deref for Receiver < C > { type Target = C ; fn deref (& self) -> & C { & self . counter () . chan } }}}
mkitem!{mkimpl!{impl < C > PartialEq for Receiver < C > { fn eq (& self , other : & Receiver < C >) -> bool { self . counter == other . counter } }}}