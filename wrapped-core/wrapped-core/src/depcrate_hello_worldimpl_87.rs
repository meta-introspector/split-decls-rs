// Generated macro for impl_87 (impl)
macro_rules! Depcrate_hello_worldimpl_87 {
() => {
// Module: crate::hello_world
// Provides: {"impl_87"}
// Dependencies: {}
impl DryDataProvider < HelloWorldV1 > for HelloWorldProvider { fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { self . load (req) . map (| r | r . metadata) } }
};
}
