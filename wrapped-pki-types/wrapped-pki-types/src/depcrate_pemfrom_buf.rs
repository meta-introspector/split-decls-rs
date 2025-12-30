// Generated macro for from_buf (function)
macro_rules! Depcrate_pemfrom_buf {
() => {
// Module: crate::pem
// Provides: {"from_buf"}
// Dependencies: {}
# [doc = " Extract and decode the next supported PEM section from `rd`."] # [doc = ""] # [doc = " - Ok(None) is returned if there is no PEM section read from `rd`."] # [doc = " - Underlying IO errors produce a `Err(...)`"] # [doc = " - Otherwise each decoded section is returned with a `Ok(Some(...))`"] # [cfg (feature = "std")] pub fn from_buf (rd : & mut dyn io :: BufRead) -> Result < Option < (SectionKind , Vec < u8 >) > , Error > { let mut b64buf = Vec :: with_capacity (1024) ; let mut line = Vec :: with_capacity (80) ; from_buf_inner (rd , & mut line , & mut b64buf) }
};
}
