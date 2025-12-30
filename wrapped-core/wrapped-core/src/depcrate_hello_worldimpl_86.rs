// Generated macro for impl_86 (impl)
macro_rules! Depcrate_hello_worldimpl_86 {
() => {
// Module: crate::hello_world
// Provides: {"impl_86"}
// Dependencies: {}
impl DataProvider < HelloWorldV1 > for HelloWorldProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < HelloWorldV1 > , DataError > { let data = Self :: DATA . iter () . find (| (l , a , _) | { req . id . locale . strict_cmp (l . as_bytes ()) . is_eq () && * a == req . id . marker_attributes . as_str () }) . map (| (_ , _ , v) | v) . ok_or_else (| | DataErrorKind :: IdentifierNotFound . with_req (HelloWorldV1 :: INFO , req)) ? ; Ok (DataResponse { metadata : DataResponseMetadata :: default () . with_checksum (1234) , payload : DataPayload :: from_static_str (data) , }) } }
};
}
