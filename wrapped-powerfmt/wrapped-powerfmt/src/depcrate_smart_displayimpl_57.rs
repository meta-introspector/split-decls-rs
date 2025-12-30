// Generated macro for impl_57 (impl)
macro_rules! Depcrate_smart_displayimpl_57 {
() => {
// Module: crate::smart_display
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a , T > Metadata < 'a , T > where T : SmartDisplay + ? Sized , { # [doc = " Creates a new `Metadata` with the given width and metadata. While the width _should_ be"] # [doc = " exact, this is not a requirement for soundness."] pub const fn new (unpadded_width : usize , _value : & T , metadata : T :: Metadata) -> Self { Self { unpadded_width , metadata , _value : PhantomData , } } # [doc = " Reuse the metadata for another type. This is useful when implementing [`SmartDisplay`] for a"] # [doc = " type that wraps another type. Both type's metadata type must be the same."] pub fn reuse < 'b , U > (self) -> Metadata < 'b , U > where 'a : 'b , U : SmartDisplay < Metadata = T :: Metadata > + ? Sized , { Metadata { unpadded_width : self . unpadded_width , metadata : self . metadata , _value : PhantomData , } } # [doc = " Obtain the width of the value before padding."] pub const fn unpadded_width (& self) -> usize { self . unpadded_width } # [doc = " Obtain the width of the value after padding."] pub fn padded_width (& self , f : FormatterOptions) -> usize { match f . width () { Some (requested_width) => cmp :: max (self . unpadded_width () , requested_width) , None => self . unpadded_width () , } } }
};
}
