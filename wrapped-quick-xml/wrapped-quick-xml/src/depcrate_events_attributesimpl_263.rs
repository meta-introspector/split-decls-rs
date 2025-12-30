// Generated macro for impl_263 (impl)
macro_rules! Depcrate_events_attributesimpl_263 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_263"}
// Dependencies: {}
impl < T > Attr < T > { # [doc = " Maps an `Attr<T>` to `Attr<U>` by applying a function to a contained key and value."] # [inline] pub fn map < U , F > (self , mut f : F) -> Attr < U > where F : FnMut (T) -> U , { match self { Attr :: DoubleQ (key , value) => Attr :: DoubleQ (f (key) , f (value)) , Attr :: SingleQ (key , value) => Attr :: SingleQ (f (key) , f (value)) , Attr :: Empty (key) => Attr :: Empty (f (key)) , Attr :: Unquoted (key , value) => Attr :: Unquoted (f (key) , f (value)) , } } }
};
}
