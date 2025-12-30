// Generated macro for impl_366 (impl)
macro_rules! Depcrate_sequenceimpl_366 {
() => {
// Module: crate::sequence
// Provides: {"impl_366"}
// Dependencies: {}
# [allow (deprecated)] impl < Input , Output , Error : ParseError < Input > , F : Parser < Input , Output = Output , Error = Error > > Tuple < Input , (Output ,) , Error > for (F ,) { fn parse_tuple (& mut self , input : Input) -> IResult < Input , (Output ,) , Error > { self . 0 . parse (input) . map (| (i , o) | (i , (o ,))) } }
};
}
