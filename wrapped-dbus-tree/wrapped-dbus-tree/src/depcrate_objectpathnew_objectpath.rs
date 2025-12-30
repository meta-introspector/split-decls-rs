// Generated macro for new_objectpath (function)
macro_rules! Depcrate_objectpathnew_objectpath {
() => {
// Module: crate::objectpath
// Provides: {"new_objectpath"}
// Dependencies: {}
pub fn new_objectpath < M : MethodType < D > , D : DataType > (n : Path < 'static > , d : D :: ObjectPath , cache : Arc < IfaceCache < M , D > >) -> ObjectPath < M , D > { ObjectPath { name : Arc :: new (n) , data : d , ifaces : ArcMap :: new () , ifacecache : cache , default_iface : None } }
};
}
