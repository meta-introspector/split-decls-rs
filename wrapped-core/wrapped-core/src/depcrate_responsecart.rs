// Generated macro for Cart (struct)
macro_rules! Depcrate_responseCart {
() => {
// Module: crate::response
// Provides: {"Cart"}
// Dependencies: {}
# [doc = " The type of the \"cart\" that is used by [`DataPayload`]."] # [doc = ""] # [doc = " This type is public but the inner cart type is private. To create a"] # [doc = " [`Yoke`] with this cart, use [`Cart::try_make_yoke`]. Then, convert"] # [doc = " it to a [`DataPayload`] with [`DataPayload::from_yoked_buffer`]."] # [derive (Clone , Debug)] pub struct Cart (# [allow (dead_code)] CartInner) ;
};
}
