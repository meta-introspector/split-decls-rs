// Generated macro for impl_404 (impl)
macro_rules! Depcrate_serverimpl_404 {
() => {
// Module: crate::server
// Provides: {"impl_404"}
// Dependencies: {}
impl ResolvesServerCert for ClientHelloResolver { fn resolve (& self , client_hello : ClientHello) -> Option < Arc < CertifiedKey > > { let server_name = client_hello . server_name () . unwrap_or_default () ; let server_name = match server_name . try_into () { Ok (r) => r , Err (_) => return None , } ; let mapped_sigs : Vec < u16 > = client_hello . signature_schemes () . iter () . map (| s | u16 :: from (* s)) . collect () ; let mapped_groups = match client_hello . named_groups () { Some (groups) => groups . iter () . map (| g | u16 :: from (* g)) . collect () , None => Vec :: new () , } ; let alpn = match client_hello . alpn () { Some (iter) => iter . collect () , None => vec ! [] , } ; let alpn = rustls_slice_slice_bytes { inner : & alpn } ; let signature_schemes = (& * mapped_sigs) . into () ; let named_groups = (& * mapped_groups) . into () ; let hello = rustls_client_hello { server_name , signature_schemes , named_groups , alpn : & alpn , } ; let cb = self . callback ; let userdata = match userdata_get () { Ok (u) => u , Err (_) => return None , } ; let key_ptr = unsafe { cb (userdata , & hello) } ; let certified_key = try_ref_from_ptr ! (key_ptr) ; Some (Arc :: new (certified_key . clone ())) } }
};
}
