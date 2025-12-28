macro_rules! deps {
    () => {
        CachedDate!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl CachedDate { fn new () -> Self { let mut cache = CachedDate { bytes : [0 ; DATE_VALUE_LENGTH] , pos : 0 , # [cfg (feature = "http2")] header_value : HeaderValue :: from_static ("") , next_update : SystemTime :: now () , } ; cache . update (cache . next_update) ; cache } fn buffer (& self) -> & [u8] { & self . bytes [..] } fn check (& mut self) { let now = SystemTime :: now () ; if now > self . next_update { self . update (now) ; } } fn update (& mut self , now : SystemTime) { let nanos = now . duration_since (UNIX_EPOCH) . unwrap_or_default () . subsec_nanos () ; self . render (now) ; self . next_update = now + Duration :: new (1 , 0) - Duration :: from_nanos (nanos as u64) ; } fn render (& mut self , now : SystemTime) { self . pos = 0 ; let _ = write ! (self , "{}" , HttpDate :: from (now)) ; debug_assert ! (self . pos == DATE_VALUE_LENGTH) ; self . render_http2 () ; } # [cfg (feature = "http2")] fn render_http2 (& mut self) { self . header_value = HeaderValue :: from_bytes (self . buffer ()) . expect ("Date format should be valid HeaderValue") ; } # [cfg (not (feature = "http2"))] fn render_http2 (& mut self) { } }
    };
}

impl_53!()