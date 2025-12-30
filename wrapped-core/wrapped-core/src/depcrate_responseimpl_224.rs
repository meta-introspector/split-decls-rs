// Generated macro for impl_224 (impl)
macro_rules! Depcrate_responseimpl_224 {
() => {
// Module: crate::response
// Provides: {"impl_224"}
// Dependencies: {}
impl Cart { # [cfg (feature = "alloc")] # [doc = " Creates a `Yoke<Y, Option<Cart>>` from owned bytes by applying `f`."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] pub fn try_make_yoke < Y , F , E > (cart : Box < [u8] > , f : F) -> Result < Yoke < Y , Option < Self > > , E > where for < 'a > Y : Yokeable < 'a > , F : FnOnce (& [u8]) -> Result < < Y as Yokeable > :: Output , E > , { Yoke :: try_attach_to_cart (SelectedRc :: new (cart) , | b | f (b)) . map (| yoke | unsafe { yoke . replace_cart (Cart) }) . map (Yoke :: wrap_cart_in_option) } # [doc = " Helper function to convert `Yoke<Y, Option<Cart>>` to `Yoke<Y, Option<CartInner>>`."] # [inline] pub (crate) fn unwrap_cart < Y > (yoke : Yoke < Y , Option < Cart > >) -> Yoke < Y , Option < CartInner > > where for < 'a > Y : Yokeable < 'a > , { unsafe { yoke . replace_cart (| option_cart | option_cart . map (| cart | cart . 0)) } } }
};
}
