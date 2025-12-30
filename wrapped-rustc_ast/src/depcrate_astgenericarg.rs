// Generated macro for GenericArg (enum)
macro_rules! Depcrate_astGenericArg {
() => {
// Module: crate::ast
// Provides: {"GenericArg"}
// Dependencies: {}
# [doc = " Concrete argument in the sequence of generic args."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum GenericArg { # [doc = " `'a` in `Foo<'a>`."] Lifetime (# [visitable (extra = LifetimeCtxt :: GenericArg)] Lifetime) , # [doc = " `Bar` in `Foo<Bar>`."] Type (Box < Ty >) , # [doc = " `1` in `Foo<1>`."] Const (AnonConst) , }
};
}
