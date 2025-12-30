// Generated macro for gen_header (function)
macro_rules! Depcrate_hpack_test_fuzzgen_header {
() => {
// Module: crate::hpack::test::fuzz
// Provides: {"gen_header"}
// Dependencies: {}
fn gen_header (g : & mut StdRng) -> Header < Option < HeaderName > > { use http :: { Method , StatusCode } ; if g . gen_ratio (1 , 10) { match g . gen_range (0u32 .. 5) { 0 => { let value = gen_string (g , 4 , 20) ; Header :: Authority (to_shared (value)) } 1 => { let method = match g . gen_range (0u32 .. 6) { 0 => Method :: GET , 1 => Method :: POST , 2 => Method :: PUT , 3 => Method :: PATCH , 4 => Method :: DELETE , 5 => { let n : usize = g . gen_range (3 .. 7) ; let bytes : Vec < u8 > = (0 .. n) . map (| _ | * g . sample (Slice :: new (b"ABCDEFGHIJKLMNOPQRSTUVWXYZ") . unwrap ())) . collect () ; Method :: from_bytes (& bytes) . unwrap () } _ => unreachable ! () , } ; Header :: Method (method) } 2 => { let value = match g . gen_range (0u32 .. 2) { 0 => "http" , 1 => "https" , _ => unreachable ! () , } ; Header :: Scheme (to_shared (value . to_string ())) } 3 => { let value = match g . gen_range (0u32 .. 100) { 0 => "/" . to_string () , 1 => "/index.html" . to_string () , _ => gen_string (g , 2 , 20) , } ; Header :: Path (to_shared (value)) } 4 => { let status = (g . gen :: < u16 > () % 500) + 100 ; Header :: Status (StatusCode :: from_u16 (status) . unwrap ()) } _ => unreachable ! () , } } else { let name = if g . gen_ratio (1 , 10) { None } else { Some (gen_header_name (g)) } ; let mut value = gen_header_value (g) ; if g . gen_ratio (1 , 30) { value . set_sensitive (true) ; } Header :: Field { name , value } } }
};
}
