// Generated macro for Compound (enum)
macro_rules! Depcrate_serCompound {
() => {
// Module: crate::ser
// Provides: {"Compound"}
// Dependencies: {}
# [doc (hidden)] pub enum Compound < 'a , W : 'a , F : 'a > { Map { ser : & 'a mut Serializer < W , F > , state : State , } , # [cfg (feature = "arbitrary_precision")] Number { ser : & 'a mut Serializer < W , F > } , # [cfg (feature = "raw_value")] RawValue { ser : & 'a mut Serializer < W , F > } , }
};
}
