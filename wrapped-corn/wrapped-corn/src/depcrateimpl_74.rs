// Generated macro for impl_74 (impl)
macro_rules! Depcrateimpl_74 {
() => {
// Module: crate
// Provides: {"impl_74"}
// Dependencies: {}
impl Display for Value < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , match self { Value :: Object (_) => "object" , Value :: Array (_) => "array" , Value :: String (_) => "string" , Value :: Integer (_) => "integer" , Value :: Float (_) => "float" , Value :: Boolean (_) => "boolean" , Value :: Null (_) => "null" , }) } }
};
}
