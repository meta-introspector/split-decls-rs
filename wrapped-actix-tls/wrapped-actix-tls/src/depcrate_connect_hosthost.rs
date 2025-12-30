// Generated macro for Host (trait)
macro_rules! Depcrate_connect_hostHost {
() => {
// Module: crate::connect::host
// Provides: {"Host"}
// Dependencies: {}
# [doc = " An interface for types where host parts (hostname and port) can be derived."] # [doc = ""] # [doc = " The [WHATWG URL Standard] defines the terminology used for this trait and its methods."] # [doc = ""] # [doc = " ```plain"] # [doc = " +------------------------+"] # [doc = " |          host          |"] # [doc = " +-----------------+------+"] # [doc = " |    hostname     | port |"] # [doc = " |                 |      |"] # [doc = " | sub.example.com : 8080 |"] # [doc = " +-----------------+------+"] # [doc = " ```"] # [doc = ""] # [doc = " [WHATWG URL Standard]: https://url.spec.whatwg.org/"] pub trait Host : Unpin + 'static { # [doc = " Extract hostname."] fn hostname (& self) -> & str ; # [doc = " Extract optional port."] fn port (& self) -> Option < u16 > { None } }
};
}
