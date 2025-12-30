// Generated macro for KeyWrapper (enum)
macro_rules! Depcrate_list_ordered_multimapKeyWrapper {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"KeyWrapper"}
// Dependencies: {}
# [doc = " A wrapper around a key that is either borrowed or owned."] # [doc = ""] # [doc = " This type is similar to [`std::borrow::Cow`] but does not require a [`Clone`] trait bound on the key."] # [allow (single_use_lifetimes)] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub enum KeyWrapper < 'map , Key > { # [doc = " An immutable reference to a key. This implies that the key is still associated to at least one value in the"] # [doc = " multimap."] Borrowed (& 'map Key) , # [doc = " An owned key. This will occur when a key is no longer associated with any values in the multimap."] Owned (Key) , }
};
}
