// Generated macro for ObjectPath (struct)
macro_rules! Depcrate_objectpathObjectPath {
() => {
// Module: crate::objectpath
// Provides: {"ObjectPath"}
// Dependencies: {}
# [derive (Debug)] # [doc = " A D-Bus Object Path."] pub struct ObjectPath < M : MethodType < D > , D : DataType > { name : Arc < Path < 'static > > , default_iface : Option < IfaceName < 'static > > , ifaces : ArcMap < Arc < IfaceName < 'static > > , Interface < M , D > > , ifacecache : Arc < IfaceCache < M , D > > , data : D :: ObjectPath , }
};
}
