// Generated macro for NUMBER_FACTORY_INSTANCE (static)
macro_rules! Depcrate_static_com_objectNUMBER_FACTORY_INSTANCE {
() => {
// Module: crate::static_com_object
// Provides: {"NUMBER_FACTORY_INSTANCE"}
// Dependencies: {}
static NUMBER_FACTORY_INSTANCE : StaticComObject < MyFactory > = MyFactory { x : AtomicU32 :: new (100) , } . into_static () ;
};
}
