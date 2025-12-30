// Generated macro for double (function)
macro_rules! Depcrate_numberdouble {
() => {
// Module: crate::number
// Provides: {"double"}
// Dependencies: {}
# [doc = " double precision floating point number parser from text"] pub fn double < T , E : ParseError < T > > () -> impl Parser < T , Output = f64 , Error = E > where T : Clone + Offset , T : Input + crate :: traits :: ParseTo < f64 > + Compare < & 'static str > , < T as Input > :: Item : AsChar + Clone , T : AsBytes , T : for < 'a > Compare < & 'a [u8] > , { Float { o : PhantomData , e : PhantomData , } }
};
}
