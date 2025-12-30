// Generated macro for float (function)
macro_rules! Depcrate_numberfloat {
() => {
// Module: crate::number
// Provides: {"float"}
// Dependencies: {}
# [doc = " single precision floating point number parser from text"] pub fn float < T , E : ParseError < T > > () -> impl Parser < T , Output = f32 , Error = E > where T : Clone + Offset , T : Input + crate :: traits :: ParseTo < f32 > + Compare < & 'static str > , < T as Input > :: Item : AsChar + Clone , T : AsBytes , T : for < 'a > Compare < & 'a [u8] > , { Float { o : PhantomData , e : PhantomData , } }
};
}
