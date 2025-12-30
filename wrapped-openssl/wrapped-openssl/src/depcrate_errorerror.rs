// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error reported from OpenSSL."] # [derive (Clone)] pub struct Error { code : ErrType , file : ShimStr , line : c_int , func : Option < ShimStr > , data : Option < Cow < 'static , str > > , }
};
}
