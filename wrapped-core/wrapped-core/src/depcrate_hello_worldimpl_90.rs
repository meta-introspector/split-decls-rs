// Generated macro for impl_90 (impl)
macro_rules! Depcrate_hello_worldimpl_90 {
() => {
// Module: crate::hello_world
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (feature = "deserialize_json")] impl DynamicDataProvider < BufferMarker > for HelloWorldJsonProvider { fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < BufferMarker > , DataError > { marker . match_marker (HelloWorldV1 :: INFO) ? ; let result = HelloWorldProvider . load (req) ? ; Ok (DataResponse { metadata : DataResponseMetadata { buffer_format : Some (icu_provider :: buf :: BufferFormat :: Json) , .. result . metadata } , # [expect (clippy :: unwrap_used)] payload : DataPayload :: from_owned_buffer (serde_json :: to_string (result . payload . get ()) . unwrap () . into_bytes () . into_boxed_slice () ,) , }) } }
};
}
