// Generated macro for impl_266 (impl)
macro_rules! Depcrate_events_attributesimpl_266 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_266"}
// Dependencies: {}
# [doc = " Unpacks attribute key and value into tuple of this two elements."] # [doc = " `None` value element is returned only for [`Attr::Empty`] variant."] impl < T > From < Attr < T > > for (T , Option < T >) { # [inline] fn from (attr : Attr < T >) -> Self { match attr { Attr :: DoubleQ (key , value) => (key , Some (value)) , Attr :: SingleQ (key , value) => (key , Some (value)) , Attr :: Empty (key) => (key , None) , Attr :: Unquoted (key , value) => (key , Some (value)) , } } }
};
}
