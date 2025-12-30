// Generated macro for impl_598 (impl)
macro_rules! Depcrate_serimpl_598 {
() => {
// Module: crate::ser
// Provides: {"impl_598"}
// Dependencies: {}
impl < 'de , S , A > SeqVisitor < 'de , S , A > where S : From < Vec < A > > , A : Deserialize < 'de > , { pub (crate) fn new () -> SeqVisitor < 'de , S , A > { SeqVisitor { phantom_s : PhantomData , phantom_a : PhantomData , phantom_lifetime : PhantomData , } } }
};
}
