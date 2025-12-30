// Generated macro for impl_983 (impl)
macro_rules! Depcrate_iter_productimpl_983 {
() => {
// Module: crate::iter::product
// Provides: {"impl_983"}
// Dependencies: {}
impl < P , T > Consumer < T > for ProductConsumer < P > where P : Send + Product < T > + Product , { type Folder = ProductFolder < P > ; type Reducer = Self ; type Result = P ; fn split_at (self , _index : usize) -> (Self , Self , Self) { (ProductConsumer :: new () , ProductConsumer :: new () , ProductConsumer :: new () ,) } fn into_folder (self) -> Self :: Folder { ProductFolder { product : iter :: empty :: < T > () . product () , } } fn full (& self) -> bool { false } }
};
}
