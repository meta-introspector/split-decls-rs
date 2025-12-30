// Generated macro for SomeIter (type)
macro_rules! Depcrate_adjSomeIter {
() => {
// Module: crate::adj
// Provides: {"SomeIter"}
// Dependencies: {}
type SomeIter < 'a , E , Ix > = core :: iter :: Map < core :: iter :: Zip < core :: iter :: Enumerate < RowIter < 'a , E , Ix > > , core :: iter :: Repeat < Ix > > , fn (((usize , & 'a WSuc < E , Ix >) , Ix)) -> EdgeReference < 'a , E , Ix > , > ;
};
}
