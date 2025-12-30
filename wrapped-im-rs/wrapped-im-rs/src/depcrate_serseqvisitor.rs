// Generated macro for SeqVisitor (struct)
macro_rules! Depcrate_serSeqVisitor {
() => {
// Module: crate::ser
// Provides: {"SeqVisitor"}
// Dependencies: {}
struct SeqVisitor < 'de , S , A > where S : From < Vec < A > > , A : Deserialize < 'de > , { phantom_s : PhantomData < S > , phantom_a : PhantomData < A > , phantom_lifetime : PhantomData < & 'de () > , }
};
}
