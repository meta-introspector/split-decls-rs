// Generated macro for MapErr (type)
macro_rules! Depcrate_resultMapErr {
() => {
// Module: crate::result
// Provides: {"MapErr"}
// Dependencies: {}
type MapErr < T , E > = statics :: Map < E , WrapErr < < T as Strategy > :: Value , < E as Strategy > :: Value > > ;
};
}
