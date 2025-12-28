macro_rules! deps {
    () => {
        WIN32_ERROR!();
        Error!();
        HRESULT!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < std :: io :: Error > for Error { fn from (from : std :: io :: Error) -> Self { match from . raw_os_error () { Some (status) => WIN32_ERROR (status as u32) . into () , None => HRESULT (E_UNEXPECTED) . into () , } } }
    };
}

impl_68!()