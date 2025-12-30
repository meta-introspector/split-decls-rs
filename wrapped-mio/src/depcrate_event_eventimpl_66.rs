// Generated macro for impl_66 (impl)
macro_rules! Depcrate_event_eventimpl_66 {
() => {
// Module: crate::event::event
// Provides: {"impl_66"}
// Dependencies: {}
# [doc = " When the [alternate] flag is enabled this will print platform specific"] # [doc = " details, for example the fields of the `kevent` structure on platforms that"] # [doc = " use `kqueue(2)`. Note however that the output of this implementation is"] # [doc = " **not** consider a part of the stable API."] # [doc = ""] # [doc = " [alternate]: fmt::Formatter::alternate"] impl fmt :: Debug for Event { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let alternate = f . alternate () ; let mut d = f . debug_struct ("Event") ; d . field ("token" , & self . token ()) . field ("readable" , & self . is_readable ()) . field ("writable" , & self . is_writable ()) . field ("error" , & self . is_error ()) . field ("read_closed" , & self . is_read_closed ()) . field ("write_closed" , & self . is_write_closed ()) . field ("priority" , & self . is_priority ()) . field ("aio" , & self . is_aio ()) . field ("lio" , & self . is_lio ()) ; if alternate { struct EventDetails < 'a > (& 'a sys :: Event) ; impl fmt :: Debug for EventDetails < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { sys :: event :: debug_details (f , self . 0) } } d . field ("details" , & EventDetails (& self . inner)) . finish () } else { d . finish () } } }
};
}
